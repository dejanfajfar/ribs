use serde::{Deserialize, Serialize};

use super::battlefield::BattleFieldContract;

#[derive(Serialize, Deserialize)]
pub struct CreateBattleContract{
    battlefield: BattleFieldContract
}

