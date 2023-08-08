use super::{
    battle_actions::BattleAction,
    battle_engine::BattleRoundState,
    combatant::Combatant,
    map::Map,
};

#[derive(Debug, Clone)]
pub struct BattleResult {
    pub combatants: Vec<Combatant>,
    pub map: Map,
    pub actions: Vec<BattleAction>,
    pub round_number: u32
}

impl BattleResult {
    pub fn new(state: BattleRoundState) -> Self {
        Self {
            combatants: state.combatants,
            map: state.map,
            actions: state.actions,
            round_number: state.round_number
        }
    }

    pub fn survivors(&self) -> Vec<Combatant> {
        self.combatants.iter().filter(|c| c.is_alive()).cloned().collect()
    }

    pub fn rounds_played(&self) -> u32 {
        self.round_number
    }
}
