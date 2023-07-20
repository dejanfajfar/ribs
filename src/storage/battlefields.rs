use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::{Entity, Record, combatants::{CombatantEntity}};

pub const COLLECTION_NAME: &'static str = "Battlefields";

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleFieldEntity {
    pub height: u8,
    pub width: u8,
    pub combatants: Vec<CombatantEntity>
}

impl Entity for BattleFieldEntity {
    fn collection_name() -> &'static str{
        COLLECTION_NAME
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleFieldRecord {
    pub height: u8,
    pub width: u8,
    pub id: Thing,
    pub combatants: Vec<CombatantEntity>
}

impl Record<BattleFieldEntity> for BattleFieldRecord {
    fn get_id(&self) -> String {
        self.id.id.to_raw()
    }

    fn get_entity(&self) -> BattleFieldEntity {
        BattleFieldEntity {
            height: self.height,
            width: self.width,
            combatants: self.combatants.to_vec()
        }
    }
}
