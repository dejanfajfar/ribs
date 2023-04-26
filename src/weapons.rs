pub mod damage;
pub mod guns;

use crate::damage::*;
use crate::guns::Gun;
use rand::{thread_rng, Rng};

use super::skills::*;

#[derive(Debug, Default)]
pub enum Weapon {
    #[default]
    None,
    Gun(Gun),
}

pub trait DmgCalculator {
    fn attack(&mut self, player_skills: Skills) -> Damage;
}

#[derive(Debug, Default)]
pub struct BaseWeaponAttributes {
    pub name: String,
    pub base_damage: u16,
    pub damage_type: DamageType,
    pub weight: u16,
}

impl BaseWeaponAttributes {
    pub fn hit_damage(&self, player_skills: Option<Skills>) -> f32 {
        let mut rng = thread_rng();
        let damage = f32::from(self.base_damage) * rng.gen_range(0.875..1.125);
        return damage.ceil();
    }
}
