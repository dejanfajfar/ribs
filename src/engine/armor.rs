use serde::{Deserialize, Serialize};

use super::damage::Damage;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Armor {
    reduction: i16,
    allow_heal: bool,
}

impl Armor {
    pub fn new(reduction: i16, allow_heal: bool) -> Self {
        return Armor {
            reduction,
            allow_heal,
        };
    }
}

impl DmgReduction for Armor {
    fn calculate_reduction(&self, incoming_damage: Damage) -> Damage {
        let dmg_reduction = Damage::from(self.reduction);
        let reduced_dmg = incoming_damage - dmg_reduction;

        if reduced_dmg < Damage::default() {
            Damage::default()
        } else {
            reduced_dmg
        }
    }
}

pub trait DmgReduction {
    fn calculate_reduction(&self, incoming_damage: Damage) -> Damage;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incoming_damage_reduced() {}

    #[test]
    fn reduce_damage_past_zero_piercing() {}

    #[test]
    fn reduce_damage_past_zero_slashing() {}

    #[test]
    fn reduce_all_damage_then_miss() {}
}
