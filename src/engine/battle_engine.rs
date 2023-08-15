use super::{
    battle_actions::BattleAction,
    battlefield::BattlefieldData,
    combatant::Combatant,
    err::Error,
    map::Map,
    battle_result::BattleResult, MAX_ROUND_NUM, combatant_turn::{CombatantTurn, CombatantTurnResult},
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

    pub fn dead_combatants(&self) -> Vec<Combatant> {
        self.combatants.iter().filter(|c| !c.is_alive()).cloned().collect::<Vec<Combatant>>()
    }

    pub fn get_combatant(&self, name: &str) -> Option<&Combatant> {
        self.combatants.iter().find(|c| c.name == name)
    }
}

impl BattleEngine {

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
            && self.round_counter <= MAX_ROUND_NUM
        {
            self.round_counter = self.round_counter + 1;

            let result: BattleRoundState =
                BattleRound::new(self.round_counter, current_battle_round_state).do_battle()?;
            current_battle_round_state = result.clone();
        }

        let battle_result = BattleResult::new(current_battle_round_state).analyze_results();

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

    pub fn do_battle(&self) -> Result<BattleRoundState, Error> {
        let mut tmp_state = self.state.clone();

        // remove dead combatants from the map!
        for dead in self.state.dead_combatants() {
            tmp_state.map = tmp_state.map.remove_poi(&dead.name);
        }

        for combatant in self.state.alive_combatants() {

            let active_combatant = tmp_state.get_combatant(&combatant.name);

            match active_combatant {
                Some(ac) => {

                    if !ac.is_alive(){
                        continue;
                    }

                    let opponents = tmp_state
                    .combatants
                    .iter()
                    .filter(|c| *c != ac)
                    .cloned()
                    .collect();
    
                let combatant_turn: CombatantTurnResult = CombatantTurn::new(
                    ac.clone(),
                    opponents,
                    tmp_state.map.clone(),
                    tmp_state.actions.to_vec(),
                    self.round_number,
                )
                .execute()?;
    
                tmp_state = BattleRoundState::from(combatant_turn);
                },
                None => continue,
            }
        }

        return Ok(tmp_state);
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


#[cfg(test)]
mod battle_round_tests {
    use super::*;

    #[test]
    #[allow(unused_must_use)]
    fn do_test(){
        let combatant1 = Combatant {
            name: String::from("Combatant1"),
            dmg: 2,
            hp: 10
        };
        let combatant2 = Combatant{
            name: String::from("Combatant2"),
            dmg: 2,
            hp: 10
        };
        let combatant3 = Combatant{
            name: String::from("Combatant3"),
            dmg: 2,
            hp: 10
        };

        let mut map = Map::new(10, 10);

        map.place_randomly(combatant1.name.clone());
        map.place_randomly(combatant2.name.clone());
        map.place_randomly(combatant3.name.clone());

        let initial_state = BattleRoundState{
            actions: vec![],
            combatants: vec![combatant1, combatant2, combatant3],
            map: map,
            round_number: 1
        };

        let test_object = BattleRound::new(2, initial_state);

        let test_result = test_object.do_battle().unwrap();

        assert_ne!(0, test_result.actions.len(), "The number of actions can not be 0")
    }
}