use std::fmt::{Display, Formatter, Result};

use crate::damage::hit::*;
use crate::damage::*;
use crate::skills::*;
use crate::weapons::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Gun {
    stats: BaseWeaponAttributes,
    clip_size: u16,
    rate_of_fire: u16,
    shots_remaining: u16,
    min_skill: Option<Skills>,
}

impl Gun {
    pub fn normalized_rate_of_fire(&self) -> u16 {
        if self.clip_size <= self.rate_of_fire {
            return self.clip_size;
        }
        return self.rate_of_fire;
    }

    pub fn reload(&mut self) {
        self.shots_remaining = self.clip_size;
    }

    pub fn is_clip_empty(&self) -> bool {
        return self.shots_remaining == 0;
    }
}

impl Display for Gun {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "<{}>, ammo: {}/{}, rof: {}",
            self.stats.name, self.shots_remaining, self.clip_size, self.rate_of_fire
        )
    }
}

impl DmgDealer for Gun {
    fn attack(&mut self, player_skills: Skills) -> Damage {
        // if the magazine is empty then we have an automatic miss
        if self.shots_remaining <= 0 {
            return Damage::Miss;
        }

        if Some(player_skills) < self.min_skill {
            return Damage::Miss;
        }

        if !player_skills.is_hit(self.min_skill) {
            self.shots_remaining -= 1;
            return Damage::Miss;
        }

        // The damage is not
        let damage: f32 = self.stats.hit_damage() * self.normalized_rate_of_fire() as f32;

        let calculated_damage: Hit = Hit::new(self.stats.damage_type, damage);

        // Firing a gun uses one bullet in the magazine.
        self.shots_remaining -= 1;

        return Damage::Hit(calculated_damage);
    }

    fn stats(&self) -> &BaseWeaponAttributes {
        return &self.stats;
    }
}

pub struct GunFactory;

impl GunFactory {
    pub fn m_10af_lexington(&self) -> Gun {
        return Gun {
            stats: BaseWeaponAttributes {
                name: String::from("m 10AF Lexington"),
                base_damage: 108,
                damage_type: DamageType::Piercing,
                weight: 2,
            },
            shots_remaining: 7,
            clip_size: 7,
            rate_of_fire: 1,
            min_skill: Some(Skills::new(3, 3)),
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_relative_eq;

    const GUN_FACTORY: GunFactory = GunFactory {};

    #[test]
    fn test_calculate_hit_damage() {
        let mut test_gun: Gun = GUN_FACTORY.m_10af_lexington();

        let test_damage = test_gun.attack(Skills::new(10, 10));

        match test_damage {
            Damage::Hit(h) => assert_relative_eq!(
                h.piercing,
                f32::from(test_gun.stats.base_damage),
                max_relative = 5.0
            ),
            Damage::Miss => assert!(false, "Did not expect a Miss at this point"),
        }
    }
}
