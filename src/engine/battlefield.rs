use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use super::damage::Damage;
use super::player::Player;
use crate::types::skillpoint::*;

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
    pub actions: Vec<BattleAction>,
    pub player_health: HashMap<String, u16>,
}

#[derive(Serialize, Deserialize)]
pub enum BattleAction {
    Attack(AttackAction),
    None(String),
}

#[derive(Serialize, Deserialize)]
pub struct AttackAction {
    attacker: String,
    target: String,
    attack_dmg: i16,
    dmg_taken: i16,
    target_hit_points_remaining: u16,
}

impl AttackAction {
    pub fn new(
        attacker: String,
        target: String,
        attack_dmg: Option<Damage>,
        dmg_taken: Damage,
        target_hit_points_remaining: u16,
    ) -> AttackAction {
        match attack_dmg {
            Some(d) => AttackAction {
                attacker,
                target,
                attack_dmg: d.damage(),
                dmg_taken: dmg_taken.damage(),
                target_hit_points_remaining,
            },
            None => AttackAction {
                attacker,
                target,
                attack_dmg: 0,
                dmg_taken: dmg_taken.damage(),
                target_hit_points_remaining,
            },
        }
    }
}

impl BattleRoundResults {
    pub fn new(round_num: u16) -> BattleRoundResults {
        return BattleRoundResults {
            round_num: round_num,
            actions: vec![],
            player_health: HashMap::default(),
        };
    }

    pub fn add_action<'a>(&mut self, action: BattleAction) {
        self.actions.push(action);
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
                let player_dmg_output: Option<Damage> = player.attack();
                let target_dmg: (Damage, u16) = t.apply_damage(player_dmg_output);
                self.add_action(BattleAction::Attack(AttackAction::new(
                    player.name(),
                    t.name(),
                    player_dmg_output,
                    target_dmg.0,
                    target_dmg.1,
                )));
                return (player, Some(t.to_owned()));
            }
            None => {
                self.add_action(BattleAction::None(player.name()));
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
