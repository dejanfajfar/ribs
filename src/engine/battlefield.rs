
use super::combatant::Combatant;

#[derive(Debug)]
pub struct BattlefieldData {
    pub battlefield_height: u8,
    pub battlefield_width: u8,
    pub combatants: Vec<Combatant>,
}

