use rocket::{http::Status, serde::json::Json};
use serde::Deserialize;

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
pub async fn get_all() -> Json<Vec<ArmorEntityRecord>> {
    let armors = ArmorEntity::get_all().await;

    match armors {
        Ok(a) => Json(a),
        Err(_) => Json(vec![]),
    }
}

#[post("/", format = "json", data = "<new_armor>")]
pub async fn create_armor(new_armor: Json<CreateArmorMessage<'_>>) -> ApiResponse {
    let new_armor = ArmorEntity::add(ArmorEntity::from(new_armor)).await;

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
pub async fn update_armor(id: &str, armor: Json<CreateArmorMessage<'_>>) -> ApiResponse {
    let updated_armor = ArmorEntity::update(id, ArmorEntity::from(armor)).await;

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
