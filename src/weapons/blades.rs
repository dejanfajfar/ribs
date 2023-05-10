use std::fmt::Display;
use std::fmt;
use std::fmt::Result;

use crate::damage::hit::*;
use crate::damage::*;
use crate::skills::*;
use crate::weapons::*;

#[derive(Debug, Default, Clone)]
pub struct Blade {
    stats: BaseWeaponAttributes,
    min_skill: Option<Skills>
}

impl DmgDealer for Blade {
    fn attack(&mut self, player_skills: Skills) -> Damage {
        if !player_skills.skill_check(self.min_skill) {
            return Damage::Miss;
        }

        if !player_skills.is_hit(self.min_skill) {
            return Damage::Miss;
        }

        return Damage::Hit(Hit::new(DamageType::Slashing, self.stats.hit_damage()));
    }

    fn stats(&self) -> &BaseWeaponAttributes {
        return &self.stats;
    }
}

impl Display for Blade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.stats.hit_damage())
    }
}

pub struct BladeFactory;

impl BladeFactory {
    pub fn katana(&self) -> Blade {
        return Blade{
            stats: BaseWeaponAttributes {
                damage_type: DamageType::Slashing,
                base_damage : 100,
                name: String::from("Katana"),
                weight: 6
            },
            min_skill: Some(Skills::new(2, 6))
        }
    }
}