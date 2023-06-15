use rocket::serde::json::Json;
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid;

use crate::storage::{*, self, entities::ArmorEntity};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateArmorMessage<'r> {
    pub name: &'r str,
    pub reduction: i16,
    pub allow_heal: bool,
}

#[get("/")]
pub async fn get_all() -> String {
    let armors: Result<Vec<ArmorEntity>, surrealdb::Error> =  get_all_armors().await;

    match armors {
        Ok(a) => serde_json::to_string(&a).unwrap(),
        Err(e) => e.to_string()
    }
}

#[post("/", data = "<create_armor_message>")]
pub async fn post_armor(create_armor_message: Json<CreateArmorMessage<'_>>) -> String {

    let new_entity = ArmorEntity {
        armor_id: uuid::Uuid::new_v4().to_string(),
        allow_heal: create_armor_message.allow_heal,
        reduction: create_armor_message.reduction,
        name: String::from(create_armor_message.name)
    };


    let new_armor = add_armor(&new_entity).await;

    match new_armor {
        Ok(a) => serde_json::to_string_pretty(&a).unwrap(),
        Err(e) => e.to_string(),
    }
}