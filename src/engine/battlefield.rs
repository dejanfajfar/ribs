use std::collections::HashMap;

use crate::{
    storage::{
        battlefields::BattleFieldRecord,
        combatants::CombatantRecord,
        Record,
    },
    types::point::Point, api::combatant::CombatantContract,
};

use super::{err::Error, combatant::Combatant};

#[derive(Debug)]
pub struct BattlefieldData {
    pub battlefield_height: u8,
    pub battlefield_width: u8,
    pub combatants: Vec<Combatant>,
}

