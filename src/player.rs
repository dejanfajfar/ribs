use crate::weapons::{damage::Damage, DmgCalculator, Weapon};

use super::skills::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct Player {
    name: String,
    skills: Skills,
    hit_points: u16,
    weapon: Weapon,
}

impl Player {
    pub fn new(name: String, skills: Skills, hit_points: u16) -> Self {
        Self {
            name,
            skills,
            hit_points,
            weapon: Weapon::None,
        }
    }

    pub fn apply_damage(&mut self, damage: Damage) -> (u16, u16) {
        match damage {
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

    pub fn add_weapon(self, weapon: Weapon) -> Self {
        Self { 
            weapon,
            ..self
        }
    }

    pub fn weapon_mut(&mut self) -> &mut Weapon {
        return &mut self.weapon;
    }

    pub fn is_alive(&self) -> bool {
        return self.hit_points > 0;
    }

    pub fn attack(&mut self) -> Damage {
        let player_skills: Skills = self.skills;
        match self.weapon_mut() {
            Weapon::None => Damage::Miss,
            Weapon::Gun(g) => {
                if g.is_clip_empty() {
                    g.reload();
                }
                g.attack(player_skills)
            },
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
