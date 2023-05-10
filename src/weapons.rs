pub mod guns;
pub mod blades;

use crate::guns::Gun;
use crate::blades::Blade;
use rand::{thread_rng, Rng};

use crate::damage::*;
use crate::skills::*;

#[derive(Debug, Clone)]
pub enum Weapon {
    Gun(Gun),
    Blade(Blade)
}

impl DmgDealer for Weapon {
    fn attack(&mut self, player_skills: Skills) -> Damage {
        match self {
            Weapon::Gun(g) => g.attack(player_skills),
            Weapon::Blade(b) => b.attack(player_skills),
        }
    }

    fn stats(&self) -> &BaseWeaponAttributes {
        match self {
            Weapon::Gun(g) => g.stats(),
            Weapon::Blade(b) => b.stats(),
        }
    }
}

pub trait DmgDealer {
    fn attack(&mut self, player_skills: Skills) -> Damage;

    fn stats(&self) -> &BaseWeaponAttributes;
}

#[derive(Debug, Default, Clone)]
pub struct BaseWeaponAttributes {
    pub name: String,
    pub base_damage: u16,
    pub damage_type: DamageType,
    pub weight: u16,
}

impl BaseWeaponAttributes {
    pub fn hit_damage(&self) -> f32 {
        let mut rng = thread_rng();
        let damage = f32::from(self.base_damage) * rng.gen_range(0.875..1.125);
        return damage.ceil();
    }
}
