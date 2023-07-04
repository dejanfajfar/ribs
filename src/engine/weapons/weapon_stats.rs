use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::engine::damage::Damage;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WeaponStats {
    pub name: String,
    pub base_damage: u16
}

impl WeaponStats {
    pub fn calculate_base_damage(&self) -> Damage {
        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let damage: f32 = f32::from(self.base_damage) * rng.gen_range(0.875..1.125);
        return Damage::from(damage.ceil());
    }
}
