use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    api::ApiResponse,
    storage::{battlefields::*, Record},
};

use super::CrudApiScaffold;

#[derive(Serialize, Deserialize)]
pub struct BattleFieldContract {
    pub height: u8,
    pub width: u8,
    pub id: Option<String>,
}

impl From<Json<BattleFieldContract>> for BattleFieldEntity {
    fn from(value: Json<BattleFieldContract>) -> Self {
        BattleFieldEntity {
            height: value.height,
            width: value.width,
        }
    }
}

impl From<&BattleFieldRecord> for BattleFieldContract {
    fn from(value: &BattleFieldRecord) -> Self {
        BattleFieldContract {
            height: value.height,
            width: value.width,
            id: Some(value.get_id()),
        }
    }
}

#[get("/")]
pub async fn get_all(db: &State<Surreal<Client>>) -> Json<Vec<BattleFieldContract>> {
    let all_battlefields: Vec<BattleFieldRecord> =
        CrudApiScaffold::get_all::<BattleFieldEntity, BattleFieldRecord>(db).await;

    return Json(Vec::from_iter(all_battlefields.iter().map(
        |record: &BattleFieldRecord| BattleFieldContract::from(record),
    )));
}

#[post("/", format = "json", data = "<post_data>")]
pub async fn create_new(
    post_data: Json<BattleFieldContract>,
    db: &State<Surreal<Client>>,
) -> ApiResponse {
    let entity: BattleFieldEntity = BattleFieldEntity::from(post_data);
    CrudApiScaffold::create_new(db, entity, |record: BattleFieldRecord| {
        BattleFieldContract::from(&record)
    })
    .await
}

#[put("/<id>", format = "json", data = "<post_data>")]
pub async fn update(
    id: &str,
    post_data: Json<BattleFieldContract>,
    db: &State<Surreal<Client>>,
) -> ApiResponse {
    let entity: BattleFieldEntity = BattleFieldEntity::from(post_data);
    CrudApiScaffold::update(db, id, entity, |record: BattleFieldRecord| {
        BattleFieldContract::from(&record)
    })
    .await
}

#[get("/<id>")]
pub async fn get_by_id(id: &str, db: &State<Surreal<Client>>) -> ApiResponse {
    CrudApiScaffold::get_by_id(db, id, |record: BattleFieldRecord| {
        BattleFieldContract::from(&record)
    })
    .await
}

#[delete("/<id>")]
pub async fn delete(id: &str, db: &State<Surreal<Client>>) -> ApiResponse {
    CrudApiScaffold::delete(db, id, |record: BattleFieldRecord| {
        BattleFieldContract::from(&record)
    })
    .await
}

//#[get("/")]
//pub fn get_battlefield() -> String {
//    let mut battlefield: BattleField = BattleField::default();
//
//    let gun_factory: GunFactory = GunFactory {};
//    let blade_factory: BladeFactory = BladeFactory {};
//    let skills_factory: SkillsFactory = SkillsFactory {};
//
//    let player1 = Player::new("Bob".to_owned(), skills_factory.random(), 400)
//        .add_weapon(Weapon::Gun(gun_factory.m_10af_lexington()));
//
//    let player3 = Player::new("Carl".to_owned(), skills_factory.random(), 400)
//        .add_weapon(Weapon::Gun(gun_factory.m_10af_lexington()));
//
//    let player2 = Player::new("Dave".to_owned(), skills_factory.ninja(), 400)
//        .add_weapon(Weapon::Blade(blade_factory.katana()))
//        .add_armor(Armor::new(10, false));
//
//    battlefield.add_player(player1);
//    battlefield.add_player(player2);
//    battlefield.add_player(player3);
//
//    let battlefield_json = serde_json::to_string_pretty(&battlefield).unwrap();
//
//    let results = battlefield.start();
//
//    let results_json = serde_json::to_string_pretty(&results).unwrap();
//
//    return results_json;
//}
