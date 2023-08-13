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
    pub round_number: u32,
    pub is_analyzed: bool,
    pub winner: Option<Combatant>
}

impl BattleResult {
    pub fn new(state: BattleRoundState) -> Self {
        Self {
            combatants: state.combatants,
            map: state.map,
            actions: state.actions,
            round_number: state.round_number,
            is_analyzed: false,
            winner: None
        }
    }

    pub fn analyze_results(mut self) -> Self {
        self.is_analyzed = true;

        // determine winner
        self.winner = self.determine_winner();

        return self;
    }

    
    fn determine_winner(&self) -> Option<Combatant> {
        let mut winner: Option<Combatant> = None;
        let survivors: Vec<Combatant> = self.combatants.to_vec().iter().cloned().filter(|c| c.is_alive()).collect();

        // In the special case that no one survived
        if survivors.len() == 0 {
            return None;
        }

        for survivor in survivors {
            match winner.clone() {
                Some(_w) => {
                    if _w.hp < survivor.hp {
                        winner = Some(survivor);
                    }
                },
                None => winner = Some(survivor),
            }
        }

        return winner;
    }
}
