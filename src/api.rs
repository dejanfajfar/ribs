use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::State;
use serde::Serialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::engine::err::Error;
use crate::storage::{Entity, GenericEntity, Record};

pub mod battlefield;
pub mod combatant;
pub mod battle;

#[derive(Debug)]
pub struct ApiResponse {
    pub json: String,
    pub status: Status,
}

impl ApiResponse {
    pub fn empty(status: Status) -> Self {
        ApiResponse { json: String::from(""), status }
    }
}

impl From<Error> for ApiResponse {
    fn from(value: Error) -> Self {
        match value {
            Error::UserAlreadyOnMap => todo!(),
            Error::LocationOccupied(_) => todo!(),
            Error::DestinationOutOfBounds(_, _) => todo!(),
            Error::MapIdUnknown(_) => todo!(),
            Error::MapLocationEmpty(_) => todo!(),
            Error::NoOpponentsPresent => todo!(),
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

pub struct CrudApiScaffold;

impl CrudApiScaffold {
    pub async fn get_all<TEntity, TRecord>(db: &State<Surreal<Client>>) -> Vec<TRecord>
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
    {
        let db_access: GenericEntity<'_> = GenericEntity::new::<TEntity>(db.inner());
        let entities: Result<Vec<TRecord>, surrealdb::Error> = db_access.get_all().await;

        match entities {
            Ok(c) => c,
            Err(_) => vec![],
        }
    }

    pub async fn get_by_id<TEntity, TRecord, TContract>(
        db: &State<Surreal<Client>>,
        id: &str,
        transformation_function: impl Fn(TRecord) -> TContract,
    ) -> ApiResponse
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
        TContract: Serialize,
    {
        let db_access: GenericEntity<'_> = GenericEntity::new::<TEntity>(db.inner());
        let entity: Result<TRecord, surrealdb::Error> = db_access.get_by_id(id).await;

        match entity {
            Ok(e) => {
                let contract: TContract = transformation_function(e);
                ApiResponse {
                    json: serde_json::to_string(&contract).unwrap(),
                    status: Status::Ok,
                }
            }
            Err(_e) => ApiResponse {
                json: String::new(),
                status: Status::NotFound,
            },
        }
    }

    pub async fn delete<TEntity, TRecord, TContract>(
        db: &State<Surreal<Client>>,
        id: &str,
        transformation_function: impl Fn(TRecord) -> TContract,
    ) -> ApiResponse
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
        TContract: Serialize,
    {
        let db_access: GenericEntity<'_> = GenericEntity::new::<TEntity>(db.inner());
        let entity = db_access.delete::<TEntity, TRecord>(id).await;

        match entity {
            Ok(e) => {
                let contract: TContract = transformation_function(e);
                ApiResponse {
                    json: serde_json::to_string(&contract).unwrap(),
                    status: Status::Ok,
                }
            }
            Err(e) => ApiResponse {
                json: e.to_string(),
                status: Status::BadRequest,
            },
        }
    }

    pub async fn create_new<TEntity, TRecord, TContract>(
        db: &State<Surreal<Client>>,
        entity: TEntity,
        transformation_function: impl Fn(TRecord) -> TContract,
    ) -> ApiResponse
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
        TContract: Serialize,
    {
        let db_access: GenericEntity<'_> = GenericEntity::new::<TEntity>(db.inner());
        let new_entity_result: Result<TRecord, surrealdb::Error> =
            db_access.create_new(entity).await;

        match new_entity_result {
            Ok(e) => {
                let contract: TContract = transformation_function(e);
                ApiResponse {
                    json: serde_json::to_string(&contract).unwrap(),
                    status: Status::Ok,
                }
            }
            Err(e) => ApiResponse {
                json: e.to_string(),
                status: Status::BadRequest,
            },
        }
    }

    pub async fn update<TEntity, TRecord, TContract>(
        db: &State<Surreal<Client>>,
        id: &str,
        entity: TEntity,
        transformation_function: impl Fn(TRecord) -> TContract,
    ) -> ApiResponse
    where
        TEntity: Entity,
        TRecord: Record<TEntity>,
        TContract: Serialize,
    {
        let db_access: GenericEntity<'_> = GenericEntity::new::<TEntity>(db.inner());
        let updated_entity_result: Result<TRecord, surrealdb::Error> =
            db_access.update(id, entity).await;

        match updated_entity_result {
            Ok(e) => {
                let contract: TContract = transformation_function(e);
                ApiResponse {
                    json: serde_json::to_string(&contract).unwrap(),
                    status: Status::Ok,
                }
            }
            Err(e) => ApiResponse {
                json: e.to_string(),
                status: Status::BadRequest,
            },
        }
    }
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
