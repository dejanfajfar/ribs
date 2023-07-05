use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

const COLLECTION_NAME: &'static str = "Combatants";

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatantEntity {
    pub name: String,
    pub damage_rating: u16,
    pub hit_points: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatantRecord {
    pub name: String,
    pub damage_rating: u16,
    pub hit_points: u16,
    pub id: Thing,
}

impl CombatantEntity {
    pub async fn get_all(db: &Surreal<Client>) -> surrealdb::Result<Vec<CombatantRecord>> {
        let combatants: Vec<CombatantRecord> = db.select(COLLECTION_NAME).await?;

        return Ok(combatants);
    }

    pub async fn crate_new(
        db: &Surreal<Client>,
        entity: CombatantEntity,
    ) -> surrealdb::Result<CombatantRecord> {
        let new_combatant: CombatantRecord = db.create(COLLECTION_NAME).content(entity).await?;

        return Ok(new_combatant);
    }

    pub async fn update(
        db: &Surreal<Client>,
        id: &str,
        entity: CombatantEntity,
    ) -> surrealdb::Result<CombatantRecord> {
        let updated_combatant: CombatantRecord =
            db.update((COLLECTION_NAME, id)).content(entity).await?;

        return Ok(updated_combatant);
    }
}
