pub mod battlefields;
pub mod combatants;
pub mod middleware;

use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub struct GenericEntity<'a> {
    db_connection: &'a Surreal<Client>,
    collection_name: String,
}

pub trait Entity: DeserializeOwned + Serialize + std::marker::Send + std::marker::Sync {
    fn collection_name() -> &'static str;
}

pub trait Record<TEntity: Entity>:
    DeserializeOwned + std::marker::Send + std::marker::Sync
{
    fn get_id(&self) -> String;
    fn get_entity(&self) -> TEntity;
}

impl<'a> GenericEntity<'a> {
    pub fn new<TEntity>(db: &'a Surreal<Client>) -> Self
    where
        TEntity: Entity,
    {
        GenericEntity {
            db_connection: db,
            collection_name: TEntity::collection_name().to_owned(),
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

    pub async fn create_new<TEntity, TRecord>(&self, entity: TEntity) -> surrealdb::Result<TRecord>
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

    pub async fn get_by_id<TEntity, TRecord>(&self, id: &str) -> surrealdb::Result<TRecord>
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
    {
        return self
            .db_connection
            .select((self.collection_name.clone(), id))
            .await;
    }

    pub async fn delete<TEntity, TRecord>(&self, id: &str) -> surrealdb::Result<TRecord>
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
    {
        return self
            .db_connection
            .delete((self.collection_name.clone(), id))
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
