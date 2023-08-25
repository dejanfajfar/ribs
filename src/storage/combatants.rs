use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing};

use super::{Entity, Record};

pub const COLLECTION_NAME: &'static str = "Combatants";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombatantEntity {
    pub name: String,
    pub damage_rating: u16,
    pub hit_points: u16,
    pub avatar: Option<String>,
}

impl Entity for CombatantEntity {
    fn collection_name() -> &'static str {
        COLLECTION_NAME
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombatantRecord {
    pub id: Thing,
    pub name: String,
    pub damage_rating: u16,
    pub hit_points: u16,
    pub avatar: Option<String>,
}

impl Record<CombatantEntity> for CombatantRecord {
    fn get_id(&self) -> String {
        self.id.id.to_raw()
    }

    fn get_entity(&self) -> CombatantEntity {
        CombatantEntity {
            name: self.name.clone(),
            damage_rating: self.damage_rating,
            hit_points: self.hit_points,
            avatar: self.avatar.clone()
        }
    }
}
