use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Surreal, engine::remote::ws::Client};

const TABLE_NAME: &str = "weapons";

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponEntity {
    pub name: String,
    pub dmg: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponEntityRecord {
    pub name: String,
    pub dmg: u16,
    pub id: Thing
}

impl WeaponEntity {
    pub async fn get_all(db: &Surreal<Client>) -> surrealdb::Result<Vec<WeaponEntityRecord>>{
        let weapons: Vec<WeaponEntityRecord> = db.select(TABLE_NAME).await?;

        return Ok(weapons);
    }

    pub async fn add(entity: WeaponEntity, db: &Surreal<Client>) -> surrealdb::Result<WeaponEntityRecord> {
        let new_weapon: WeaponEntityRecord = db.create((TABLE_NAME, entity.name.clone()))
        .content(entity)
        .await?;

        return Ok(new_weapon);
    }
}