pub mod armor_store;
pub mod combatants;
pub mod middleware;
pub mod weapon_store;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealdb::sql::Id;

pub struct GenericEntity<'a> {
    db_connection: &'a Surreal<Client>,
    collection_name: String,
}

trait Entity: DeserializeOwned + Serialize + std::marker::Send + std::marker::Sync {}

pub trait Record<TEntity: Entity>:
    DeserializeOwned + std::marker::Send + std::marker::Sync
{
    fn get_id(&self) -> String;
    fn get_entity(&self) -> &TEntity;
}

impl<'a> GenericEntity<'a> {
    pub fn new(db: &'a Surreal<Client>, collection: &'a str) -> Self {
        GenericEntity {
            db_connection: db,
            collection_name: collection.to_owned(),
        }
    }

    pub async fn get_all<TEntity, TRecord>(&self) -> surrealdb::Result<Vec<TRecord>>
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
    {
        return self
            .db_connection
            .select(self.collection_name.clone())
            .await;
    }

    pub async fn create_new<TEntity, TRecord>(
        &self,
        entity: TEntity,
    ) -> surrealdb::Result<TRecord>
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
    {
        return self
            .db_connection
            .create(self.collection_name.clone())
            .content(entity)
            .await;
    }

    pub async fn update<TEntity, TRecord>(
        &self,
        id: &str,
        entity: TEntity,
    ) -> surrealdb::Result<TRecord>
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
    {
        return self
            .db_connection
            .update((self.collection_name.clone(), id))
            .content(entity)
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
