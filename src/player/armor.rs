use crate::damage::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Armor {
    reduction: i16,
}

impl Armor {
    pub fn new(reduction: i16) -> Self {
        return Armor { reduction };
    }

    pub fn calculate_reduction(&self, incoming_damage: Damage) -> Damage {
        let dmg_reduction = Damage::from(self.reduction);
        return incoming_damage - dmg_reduction;
    }
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
