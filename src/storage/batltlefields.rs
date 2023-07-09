use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

const COLLECTION_NAME: &'static str = "Battlefields";

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleFieldEntity {
    pub height: u8,
    pub width: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleFieldRecord {
    pub height: u8,
    pub width: u8,
    pub id: Thing,
}

impl BattleFieldEntity {
    pub async fn get_all(db: &Surreal<Client>) -> surrealdb::Result<Vec<BattleFieldRecord>> {
        let combatants: Vec<BattleFieldRecord> = db.select(COLLECTION_NAME).await?;

        return Ok(combatants);
    }

    pub async fn crate_new(
        db: &Surreal<Client>,
        entity: BattleFieldEntity,
    ) -> surrealdb::Result<BattleFieldRecord> {
        let new_combatant: BattleFieldRecord = db.create(COLLECTION_NAME).content(entity).await?;

        return Ok(new_combatant);
    }

    pub async fn update(
        db: &Surreal<Client>,
        id: &str,
        entity: BattleFieldEntity,
    ) -> surrealdb::Result<BattleFieldRecord> {
        let updated_combatant: BattleFieldRecord =
            db.update((COLLECTION_NAME, id)).content(entity).await?;

        return Ok(updated_combatant);
    }
}
