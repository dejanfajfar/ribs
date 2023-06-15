use surrealdb::sql::Thing;
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorEntity {
    pub armor_id: String,
    pub name: String,
    pub reduction: i16,
    pub allow_heal: bool,
}