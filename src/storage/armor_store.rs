use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorEntity {
    pub name: String,
    pub reduction: i16,
    pub allow_heal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorEntityRecord {
    pub name: String,
    pub reduction: i16,
    pub allow_heal: bool,
    pub id: Thing,
}

impl ArmorEntity {
    pub async fn get_all() -> surrealdb::Result<Vec<ArmorEntityRecord>> {
        let db = super::connect_to_db().await?;

        let armors: Vec<ArmorEntityRecord> = db.select("armors").await?;

        return Ok(armors);
    }

    pub async fn add(entity: ArmorEntity) -> surrealdb::Result<ArmorEntityRecord> {
        let db = super::connect_to_db().await?;

        let new_armor: ArmorEntityRecord = db
            .create("armors")
            .content(entity)
            .await?;

        return Ok(new_armor);
    }

    pub async fn update(id : &str, entity: ArmorEntity) -> surrealdb::Result<ArmorEntityRecord> {
        let db = super::connect_to_db().await?;

        let updated_armor = db.update(("armors", id))
        .content(entity)
        .await?;

        return Ok(updated_armor);
    }
}