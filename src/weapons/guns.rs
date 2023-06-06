use crate::damage::*;
use crate::skills::*;
use crate::weapons::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Gun {
    stats: WeaponStats,
    clip_size: u16,
    rate_of_fire: u8,
    shots_remaining: u16,
    min_skill: Option<Skills>,
}

impl Gun {
    pub fn normalized_rate_of_fire(&self) -> i32 {
        if self.clip_size <= u16::from(self.rate_of_fire) {
            return i32::from(self.clip_size);
        }
        return i32::from(self.rate_of_fire);
    }

    pub fn reload(&mut self) {
        self.shots_remaining = self.clip_size;
    }

    pub fn is_clip_empty(&self) -> bool {
        return self.shots_remaining == 0;
    }
}

impl DmgDealer for Gun {
    fn attack(&mut self, player_skills: Skills) -> Option<Damage> {
        // if the magazine is empty then we have an automatic miss
        if self.shots_remaining <= 0 {
            return None;
        }

        if Some(player_skills) < self.min_skill {
            return None;
        }

        if !player_skills.is_hit(self.min_skill) {
            self.shots_remaining -= 1;
            return None;
        }

        // The damage is not
        let damage = self.stats.calculate_base_damage() * self.normalized_rate_of_fire();

        // Firing a gun uses one bullet in the magazine.
        self.shots_remaining -= 1;

        return Some(damage);
    }

    fn stats(&self) -> &WeaponStats {
        return &self.stats;
    }
}

pub struct GunFactory;

impl GunFactory {
    pub fn m_10af_lexington(&self) -> Gun {
        return Gun {
            stats: WeaponStats {
                name: String::from("m 10AF Lexington"),
                base_damage: 108,
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
    fn test_calculate_hit_damage() {}
}
