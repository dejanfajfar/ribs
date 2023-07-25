use crate::{types::point::Point, storage::combatants::CombatantRecord};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Combatant{
    pub name: String,
    pub hp: u16,
    pub dmg: u16,
}

impl Combatant {
    pub fn is_alive(&self) -> bool {
        self.hp != u16::MIN
    }

    pub fn apply_damage(&mut self, damage: u16) {
        let sub_result = self.hp.overflowing_sub(damage);

        if sub_result.1 {
            self.hp = u16::MIN;
        }
        else {
            self.hp = sub_result.0;
        }
    }
}