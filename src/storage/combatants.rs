use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use super::{Record, Entity};

pub const COLLECTION_NAME: &'static str = "Combatants";

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatantEntity {
    pub name: String,
    pub damage_rating: u16,
    pub hit_points: u16,
}

impl Entity for CombatantEntity{}

#[derive(Debug, Serialize)]
pub struct CombatantRecord {
    pub id: Thing,
    pub entity: CombatantEntity
}

impl Record<CombatantEntity> for CombatantRecord {
    fn get_id(&self) -> String {
        self.id.id.to_raw()
    }

    fn get_entity(&self) -> &CombatantEntity {
        &self.entity
    }
}

// ToDo: Implement visitor for CombatantEntitiy => https://serde.rs/impl-deserialize.html#the-visitor-trait