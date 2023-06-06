pub mod blades;
pub mod guns;
pub mod weapon_stats;

use crate::blades::Blade;
use crate::guns::Gun;
use crate::weapon_stats::WeaponStats;

use serde::{Deserialize, Serialize};

use crate::damage::*;
use crate::skills::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weapon {
    Gun(Gun),
    Blade(Blade),
}

impl DmgDealer for Weapon {
    fn attack(&mut self, player_skills: Skills) -> Option<Damage> {
        match self {
            Weapon::Gun(g) => g.attack(player_skills),
            Weapon::Blade(b) => b.attack(player_skills),
        }
    }

    fn stats(&self) -> &WeaponStats {
        match self {
            Weapon::Gun(g) => g.stats(),
            Weapon::Blade(b) => b.stats(),
        }
    }
}

pub trait DmgDealer {
    fn attack(&mut self, player_skills: Skills) -> Option<Damage>;

    fn stats(&self) -> &WeaponStats;
}
