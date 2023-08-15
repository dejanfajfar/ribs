pub mod battlefield;
pub mod movement;
pub mod err;
pub mod battle_engine;
pub mod combatant;
pub mod map;
pub mod battle_actions;
pub mod battle_result;
pub mod combatant_turn;

 // the maximal number of rounds to be played
 pub const MAX_ROUND_NUM: u32 = 1000;

 // The default number of steps each combatant can take
 pub const MAX_COMBATANT_MOVE: usize = 3;