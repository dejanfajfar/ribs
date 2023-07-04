use serde::{Deserialize, Serialize};

use crate::engine::{damage::Damage};
use crate::types::skills::*;

use super::{weapon_stats::WeaponStats, DmgDealer};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Blade {
    stats: WeaponStats,
    min_skill: Option<Skills>,
}

impl DmgDealer for Blade {
    fn attack(&mut self, player_skills: Skills) -> Option<Damage> {
        if Some(player_skills) < self.min_skill {
            return None;
        }

        if !player_skills.is_hit(self.min_skill) {
            return None;
        }

        return Some(self.stats.calculate_base_damage());
    }

    fn stats(&self) -> &WeaponStats {
        return &self.stats;
    }
}

pub struct BladeFactory;

impl BladeFactory {
    pub fn katana(&self) -> Blade {
        return Blade {
            stats: WeaponStats {
                base_damage: 100,
                name: String::from("Katana"),
            },
            min_skill: Some(Skills::new(2, 6)),
        };
    }
}
