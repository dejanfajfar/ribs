use crate::damage::*;
use crate::skills::*;
use crate::weapons::{DmgDealer, Weapon};
use serde::{Deserialize, Serialize};

pub mod armor;

use armor::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn apply_damage(&mut self, damage: Option<Damage>) -> (Damage, u16) {
        match damage {
            None => (Damage::default(), self.hit_points),
            Some(d) => {
                let reduced_damage = self.reduce_damage(d);

                if reduced_damage >= Damage::default() {
                    let dmg = reduced_damage.damage() as u16;

                    if self.hit_points <= dmg {
                        self.hit_points = 0;
                        return (reduced_damage, 0);
                    } else {
                        self.hit_points = self.hit_points - dmg;
                        return (reduced_damage, self.hit_points);
                    }
                } else {
                    let dmg = reduced_damage.damage().abs() as u16;

                    self.hit_points = self.hit_points + dmg;
                    return (reduced_damage, self.hit_points);
                }
            }
        }
    }

    fn reduce_damage(&self, damage: Damage) -> Damage {
        match &self.armor {
            None => return damage,
            Some(a) => return a.calculate_reduction(damage),
        };
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn add_weapon(self, weapon: Weapon) -> Self {
        Self {
            weapon: Some(weapon),
            ..self
        }
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

    pub fn attack(&mut self) -> Option<Damage> {
        let player_skills: Skills = self.skills;
        match self.weapon_mut() {
            None => None,
            Some(Weapon::Gun(g)) => {
                if g.is_clip_empty() {
                    g.reload();
                }
                g.attack(player_skills)
            }
            Some(Weapon::Blade(b)) => b.attack(player_skills),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
