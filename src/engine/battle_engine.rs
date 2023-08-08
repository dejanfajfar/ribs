use rand::seq::SliceRandom;

use crate::types::point::Point;

use super::{
    battle_actions::{BattleAction, BattleAttackAction},
    battlefield::BattlefieldData,
    combatant::Combatant,
    err::Error,
    map::Map,
    movement::{MovementEngine, MovementResult}, battle_result::BattleResult,
};

pub struct BattleEngine {
    map: Map,
    combatants: Vec<Combatant>,
    round_counter: u32,
}

#[derive(Debug, Clone)]
pub struct BattleRound {
    round_number: u32,
    state: BattleRoundState,
}

#[derive(Debug, Clone)]
pub struct BattleRoundState {
    pub combatants: Vec<Combatant>,
    pub map: Map,
    pub actions: Vec<BattleAction>,
    pub round_number: u32
}

#[derive(Debug, Clone)]
pub struct CombatantTurn {
    active_combatant: Combatant,
    opponents: Vec<Combatant>,
    map: Map,
    actions: Vec<BattleAction>,
    round_number: u32,
}

pub struct CombatantTurnResult {
    pub active_combatant: Combatant,
    pub opponents: Vec<Combatant>,
    pub map: Map,
    pub actions: Vec<BattleAction>,
    pub round_number: u32
}

impl BattleRoundState {
    pub fn min_two_alive(&self) -> bool {
        self.combatants
            .iter()
            .filter(|c| c.is_alive())
            .collect::<Vec<&Combatant>>()
            .len()
            >= 2
    }

    pub fn alive_combatants(&self) -> Vec<Combatant> {
        self.combatants.iter().filter(|c| c.is_alive()).cloned().collect::<Vec<Combatant>>()
    }
}

impl BattleEngine {
    // the maximal number of rounds to be played
    pub const MAX_ROUND_NUM: u32 = 100;

    // The default number of steps each combatant can take
    pub const MAX_COMBATANT_MOVE: usize = 3;

    pub fn new(battlefield_data: BattlefieldData) -> Result<Self, Error> {
        let mut instance = BattleEngine { 
            map: Map::new(
                battlefield_data.battlefield_width,
                battlefield_data.battlefield_height,
            ),
            combatants: battlefield_data.combatants.to_vec(),
            round_counter: u32::MIN,
        };

        for com in battlefield_data.combatants.clone() {
            instance.map.place_randomly(com.name)?;
        }

        return Ok(instance);
    }

    pub fn start_battle(&mut self) -> Result<BattleResult, Error> {
        let mut current_battle_round_state: BattleRoundState = BattleRoundState {
            combatants: self.combatants.to_vec(),
            map: self.map.clone(),
            actions: vec![],
            round_number: self.round_counter
        };

        while current_battle_round_state.min_two_alive()
            && self.round_counter <= BattleEngine::MAX_ROUND_NUM
        {
            self.round_counter = self.round_counter + 1;

            let result: BattleRoundState =
                BattleRound::new(self.round_counter, current_battle_round_state).do_battle()?;
            current_battle_round_state = result.clone();
        }

        let battle_result = BattleResult::new(current_battle_round_state);

        return Ok(battle_result);
    }
}

impl BattleRound {
    pub fn new(round_number: u32, initial_state: BattleRoundState) -> Self {
        BattleRound {
            round_number: round_number,
            state: initial_state,
        }
    }

    pub fn do_battle(&mut self) -> Result<BattleRoundState, Error> {
        for combatant in self.state.alive_combatants() {

            let active_combatant = combatant;
            let opponents = self
                .state
                .combatants
                .iter()
                .filter(|c| **c != active_combatant)
                .cloned()
                .collect();

            let combatant_turn: CombatantTurnResult = CombatantTurn::new(
                active_combatant,
                opponents,
                self.state.map.clone(),
                self.state.actions.to_vec(),
                self.round_number,
            )
            .execute()?;

            self.state = BattleRoundState::from(combatant_turn);
        }

        return Ok(self.state.clone());
    }
}

impl From<CombatantTurnResult> for BattleRoundState {
    fn from(value: CombatantTurnResult) -> Self {
        let mut all_combatants = value.opponents.to_vec();
        all_combatants.push(value.active_combatant);
        BattleRoundState {
            combatants: all_combatants,
            map: value.map.clone(),
            actions: value.actions.to_vec(),
            round_number: value.round_number
        }
    }
}

impl From<&mut CombatantTurn> for CombatantTurnResult {
    fn from(value: &mut CombatantTurn) -> Self {
        Self {
            active_combatant: value.active_combatant.clone(),
            opponents: value.opponents.clone(),
            map: value.map.clone(),
            actions: value.actions.to_vec(),
            round_number: value.round_number
        }
    }
}

impl CombatantTurn {
    pub fn new(
        active: Combatant,
        opponents: Vec<Combatant>,
        map: Map,
        actions: Vec<BattleAction>,
        round_number: u32,
    ) -> Self {
        Self {
            active_combatant: active,
            opponents,
            map,
            actions,
            round_number,
        }
    }

    pub fn execute(&mut self) -> Result<CombatantTurnResult, Error> {
        // If no opponents are present then we can not have an Combatant turn
        if self.opponents.len() == 0 {
            return Err(Error::NoOpponentsPresent);
        }

        let active_combatant_position: Option<Point> =
            self.map.position_for(&self.active_combatant.name);

        match active_combatant_position {
            Some(active_position) => {
                // Move the active combatant to the closest opponent
                let movement: MovementResult = MovementEngine::new(
                    active_position,
                    self.opponents_locations(),
                    Some(BattleEngine::MAX_COMBATANT_MOVE),
                )
                .do_move();

                if movement.has_moved() {
                    self.actions.push(BattleAction::Move(
                        self.round_number,
                        self.active_combatant.name.clone(),
                        movement.clone(),
                    ));

                    // update the active combatants position on the map
                    self.map.move_to(active_position, movement.last_position)?;
                }

                // Determine if any opponent is in range
                let mut potential_targets: Vec<String> =
                    self.map.get_occupied_neighbors(movement.last_position);
                potential_targets.shuffle(&mut rand::thread_rng());
                let opponent_id: Option<&String> = potential_targets.first();

                match opponent_id {
                    Some(id) => {
                        self.attack(id.clone());
                        return Ok(CombatantTurnResult::from(self));
                    }
                    None => Ok(CombatantTurnResult::from(self)),
                }
            }
            // The map does not know the position of the active combatant
            None => Ok(CombatantTurnResult::from(self)),
        }
    }

    fn opponents_locations(&self) -> Vec<Point> {
        self.opponents
            .iter()
            .map(|c| self.map.position_for(&c.name))
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .collect()
    }

    fn attack(&mut self, opponent_id: String) {
        let cloned = self.opponents.to_vec();
        self.opponents.clear();

        for mut opponent in cloned {
            if opponent.name == opponent_id {
                opponent.apply_damage(self.active_combatant.dmg);

                // Add a protocol of who is attacking who and for how much
                self.actions.push(BattleAction::Attack(
                    self.round_number,
                    BattleAttackAction {
                        assailant: self.active_combatant.clone(),
                        victim: opponent.clone(),
                        damage: self.active_combatant.dmg,
                    },
                ));
            }

            self.opponents.push(opponent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_battle() {
        let engine = BattleEngine::new(BattlefieldData {
            battlefield_height: 10,
            battlefield_width: 10,
            combatants: vec![
                Combatant {
                    dmg: 2,
                    hp: 15,
                    name: "test1".to_owned(),
                },
                Combatant {
                    dmg: 4,
                    hp: 10,
                    name: "test2".to_owned(),
                },
                Combatant {
                    dmg: 2,
                    hp: 15,
                    name: "test3".to_owned(),
                },
            ],
        });

        let results = engine.unwrap().start_battle();

        assert!(results.is_ok());

        let r = results.unwrap();

        let foo = r.actions.iter().any(|a| match a {
            BattleAction::Move(_, _, _) => false,
            BattleAction::Attack(_r, attack) => attack.victim.hp == 0,
        });

        assert!(foo);

        assert_eq!(3, r.combatants.len());
    }
}
