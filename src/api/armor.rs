use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::storage::armor_store::{ArmorEntity, ArmorEntityRecord};

use super::ApiResponse;

#[derive(Deserialize)]
pub struct CreateArmorMessage<'r> {
    pub name: &'r str,
    pub reduction: i16,
    pub allow_heal: bool,
}

impl From<Json<CreateArmorMessage<'_>>> for ArmorEntity {
    fn from(value: Json<CreateArmorMessage<'_>>) -> Self {
        ArmorEntity {
            name: String::from(value.name),
            reduction: value.reduction,
            allow_heal: value.allow_heal,
        }
    }
}

#[get("/")]
pub async fn get_all(db: &State<Surreal<Client>>) -> Json<Vec<ArmorEntityRecord>> {
    let armors: Result<Vec<ArmorEntityRecord>, surrealdb::Error> = ArmorEntity::get_all(db.inner()).await;

    match armors {
        Ok(a) => Json(a),
        Err(_) => Json(vec![]),
    }
}

#[post("/", format = "json", data = "<new_armor>")]
pub async fn create_armor(new_armor: Json<CreateArmorMessage<'_>>, db: &State<Surreal<Client>>) -> ApiResponse {
    let new_armor: Result<ArmorEntityRecord, surrealdb::Error> = ArmorEntity::add(ArmorEntity::from(new_armor), db.inner()).await;

    match new_armor {
        Ok(a) => ApiResponse {
            json: serde_json::to_string(&a).unwrap(),
            status: Status::Ok,
        },
        Err(e) => ApiResponse {
            json: e.to_string(),
            status: Status::BadRequest,
        },
    }
}

#[post("/<id>", format = "json", data = "<armor>")]
pub async fn update_armor(id: &str, armor: Json<CreateArmorMessage<'_>>, db: &State<Surreal<Client>>) -> ApiResponse {
    let updated_armor: Result<ArmorEntityRecord, surrealdb::Error> = ArmorEntity::update(id, ArmorEntity::from(armor), db.inner()).await;

    match updated_armor {
        Ok(a) => ApiResponse {
            json: serde_json::to_string(&a).unwrap(),
            status: Status::Ok,
        },
        Err(e) => ApiResponse {
            json: e.to_string(),
            status: Status::BadRequest,
        },
    }
}
