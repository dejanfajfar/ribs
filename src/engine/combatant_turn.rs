use rand::seq::SliceRandom;

use crate::types::point::Point;

use super::{
    battle_actions::{BattleAction, BattleAttackAction},
    combatant::Combatant,
    err::Error,
    map::Map,
    movement::{MovementEngine, MovementResult},
    MAX_COMBATANT_MOVE,
};

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
    pub round_number: u32,
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
                    Some(MAX_COMBATANT_MOVE),
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
            //.filter(|c| c.is_alive())
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

impl From<&mut CombatantTurn> for CombatantTurnResult {
    fn from(value: &mut CombatantTurn) -> Self {
        Self {
            active_combatant: value.active_combatant.clone(),
            opponents: value.opponents.clone(),
            map: value.map.clone(),
            actions: value.actions.to_vec(),
            round_number: value.round_number,
        }
    }
}

#[cfg(test)]
mod combatant_turn_tests {
    use super::*;

    #[test]
    #[allow(unused_must_use)] // alow the test code to ignore return values
    fn do_test() {
        let active_combattant = Combatant {
            name: String::from("Active"),
            dmg: 2,
            hp: 10,
        };
        let opponent1 = Combatant {
            name: String::from("Opponent1"),
            dmg: 2,
            hp: 10,
        };
        let opponent2 = Combatant {
            name: String::from("Opponent2"),
            dmg: 2,
            hp: 10,
        };

        let mut map = Map::new(10, 10);

        map.place_randomly(active_combattant.name.clone());
        map.place_randomly(opponent1.name.clone());
        map.place_randomly(opponent2.name.clone());

        let mut test_object: CombatantTurn = CombatantTurn::new(
            active_combattant,
            vec![opponent1, opponent2],
            map,
            vec![],
            1,
        );

        let test_result = test_object.execute().unwrap();

        assert_ne!(
            0,
            test_result.actions.len(),
            "The number of actions can not be 0"
        )
    }
}
