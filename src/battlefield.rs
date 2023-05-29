use std::collections::HashMap;

use crate::damage::Damage;
use crate::player::Player;
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BattleField {
    players: HashMap<String, Player>,
}

#[derive(Serialize, Deserialize)]
pub struct BattleResult {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub rounds: Vec<BattleRoundResults>,
}

#[derive(Serialize, Deserialize)]
pub struct BattleRoundResults {
    pub round_num: u16,
    pub actions: Vec<BattleRoundAction>,
}

#[derive(Serialize, Deserialize)]
pub enum BattleRoundAction {
    Attack(String, String, u16),
    None(String),
}

impl BattleRoundResults {
    pub fn new(round_num: u16) -> BattleRoundResults {
        return BattleRoundResults {
            round_num: round_num,
            actions: vec![],
        };
    }

    pub fn add_action<'a>(
        &mut self,
        player_identifier: String,
        target_identifier: String,
        dmg_taken: u16,
    ) {
        self.actions.push(BattleRoundAction::Attack(
            player_identifier,
            target_identifier,
            dmg_taken,
        ));
    }

    pub fn add_inaction(&mut self, player_identifier: String) {
        self.actions
            .push(BattleRoundAction::None(player_identifier));
    }

    pub fn take_player_action(
        &mut self,
        mut player: Player,
        mut targets: Vec<Player>,
    ) -> (Player, Option<Player>) {
        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let target = targets.choose_mut(&mut rng);

        match target {
            Some(t) => {
                let player_dmg_output: Damage = player.attack();
                let target_dmg: (u16, u16) = t.apply_damage(player_dmg_output);
                self.add_action(player.name(), t.name(), target_dmg.0);
                return (player, Some(t.to_owned()));
            }
            None => {
                self.add_inaction(player.name());
                return (player, None);
            }
        }
    }
}

impl BattleField {
    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.name(), player);
    }

    pub fn start(&mut self) -> BattleResult {
        let mut loop_guard: i32 = 100;
        let _start: DateTime<Utc> = Utc::now();
        let mut _rounds: Vec<BattleRoundResults> = vec![];
        let mut round_counter: u16 = 1;

        while self.players.iter().take_while(|p| p.1.is_alive()).count() > 1 && loop_guard > 0 {
            _rounds.push(self.do_round(round_counter));
            round_counter += 1;
            loop_guard -= 1;
        }

        return BattleResult {
            start: _start,
            end: Utc::now(),
            rounds: _rounds,
        };
    }

    fn do_round(&mut self, round_num: u16) -> BattleRoundResults {
        let mut round_result: BattleRoundResults = BattleRoundResults::new(round_num);
        let mut _instance_players = self.players.clone();
        for player in _instance_players.iter_mut().take_while(|p| p.1.is_alive()) {
            let _targets = Vec::from_iter(
                self.players
                    .iter()
                    .take_while(|p| p.1.name() != player.1.name() && p.1.is_alive())
                    .map(|p| p.1)
                    .cloned(),
            );
            // Attack phase
            let altered_players = round_result.take_player_action(player.1.to_owned(), _targets);

            self.update_player_state(altered_players.0);

            match altered_players.1 {
                Some(p) => self.update_player_state(p),
                None => continue,
            }
        }

        return round_result;
    }

    fn update_player_state(&mut self, player: Player) {
        self.players.insert(player.name(), player);
    }
}

impl Default for BattleField {
    fn default() -> Self {
        Self {
            players: Default::default(),
        }
    }
}
