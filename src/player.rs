use crate::damage::*;
use crate::skills::*;
use crate::weapons::{DmgDealer, Weapon};

pub mod armor;

use armor::*;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    skills: Skills,
    hit_points: u16,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
}

impl Player {
    pub fn new(name: String, skills: Skills, hit_points: u16) -> Self {
        Self {
            name,
            skills,
            hit_points,
            weapon: None,
            armor: None,
        }
    }

    pub fn apply_damage(&mut self, damage: Damage) -> (u16, u16) {
        match damage {
            Damage::Miss => (0, self.hit_points),
            Damage::Hit(_h) => {
                let reduced_damage = self.reduce_damage(damage);
                match reduced_damage {
                    Damage::Miss => (0, self.hit_points),
                    Damage::Hit(h) => {
                        let combined_damage: u16 = (h.piercing + h.slashing).floor() as u16;

                        if self.hit_points <= combined_damage {
                            self.hit_points = 0;
                            return (combined_damage, 0);
                        } else {
                            self.hit_points = self.hit_points - combined_damage;
                            return (combined_damage, self.hit_points);
                        }
                    }
                }
            }
        }
    }

    fn reduce_damage(&self, damage: Damage) -> Damage {
        match &self.armor {
            None => return damage,
            Some(a) => return a.calculate_reduction(&damage),
        };
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn add_weapon(self, weapon: Weapon) -> Self {
        Self { weapon: Some(weapon), ..self }
    }

    pub fn add_armor(self, armor: Armor) -> Self {
        return Self {
            armor: Some(armor),
            ..self
        };
    }

    pub fn weapon_mut(&mut self) -> &mut Option<Weapon> {
        return &mut self.weapon;
    }

    pub fn is_alive(&self) -> bool {
        return self.hit_points > 0;
    }

    pub fn attack(&mut self) -> Damage {
        let player_skills: Skills = self.skills;
        match self.weapon_mut() {
            None => Damage::Miss,
            Some(Weapon::Gun(g)) => {
                if g.is_clip_empty() {
                    g.reload();
                }
                g.attack(player_skills)
            }
            Some(Weapon::Blade(b)) => {
                b.attack(player_skills)
            }
        }
    }

    pub fn pretty_print(&self) -> String {
        let mut print_out: String = String::from("");

        print_out.push_str(format!("{:=^1$}", self.name, 40).as_str());
        print_out.push('\n');
        print_out.push_str(format!("HP : {}", self.hit_points).as_str());
        print_out.push('\n');
        print_out.push_str(format!("Strength : {0:<5} Dexterity : {1:<5}", self.skills.strength(), self.skills.dexterity()).as_str());
        print_out.push('\n');
        print_out.push_str(format!("{:-^1$}", "Weapon", 40).as_str());
        print_out.push('\n');

        if self.weapon.is_some(){
            let weapon: &Weapon =  self.weapon.as_ref().ok_or("no weapon equipped").unwrap();
            print_out.push_str(format!("Name: {}", weapon.stats().name).as_str());
            print_out.push('\n');
            print_out.push_str(format!("Base damage: {0:<5} Damage type : {1}", weapon.stats().base_damage, weapon.stats().damage_type).as_str());
        }
        else {
            print_out.push_str("No weapon equipped");
        }

        return print_out;
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
