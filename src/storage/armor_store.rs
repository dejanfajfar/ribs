use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Surreal, engine::remote::ws::Client};

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
    pub async fn get_all(db: &Surreal<Client>) -> surrealdb::Result<Vec<ArmorEntityRecord>> {

        let armors: Vec<ArmorEntityRecord> = db.select("armors").await?;

        return Ok(armors);
    }

    pub async fn add(entity: ArmorEntity, db: &Surreal<Client>) -> surrealdb::Result<ArmorEntityRecord> {
        let new_armor: ArmorEntityRecord = db
            .create(("armors", entity.name.clone()))
            .content(entity)
            .await?;

        return Ok(new_armor);
    }

    pub async fn update(id : &str, entity: ArmorEntity, db: &Surreal<Client>) -> surrealdb::Result<ArmorEntityRecord> {
        let updated_armor = db.update(("armors", id))
            .content(entity)
            .await?;

        return Ok(updated_armor);
    }
}