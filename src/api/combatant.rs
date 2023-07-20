use crate::storage::{combatants::*, Record};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use super::{ApiResponse, CrudApiScaffold};

#[derive(Deserialize, Serialize, Clone)]
pub struct CombatantContract {
    pub name: String,
    pub id: Option<String>,
    pub hp: u16,
    pub dmg: u16,
}

impl From<&CombatantRecord> for CombatantContract {
    fn from(value: &CombatantRecord) -> Self {
        let entity: CombatantEntity = value.get_entity();
        CombatantContract {
            name: entity.name.clone(),
            id: Some(value.get_id()),
            hp: entity.hit_points,
            dmg: entity.damage_rating,
        }
    }
}

impl From<&CombatantEntity> for CombatantContract {
    fn from(value: &CombatantEntity) -> Self {
        CombatantContract {
            name: value.name.clone(),
            id: None,
            hp: value.hit_points,
            dmg: value.damage_rating,
        }
    }
}

impl From<Json<CombatantContract>> for CombatantEntity {
    fn from(value: Json<CombatantContract>) -> Self {
        CombatantEntity {
            name: value.name.clone(),
            damage_rating: value.dmg,
            hit_points: value.hp,
        }
    }
}

#[get("/")]
pub async fn get_all(db: &State<Surreal<Client>>) -> Json<Vec<CombatantContract>> {
    let all_combatants: Vec<CombatantRecord> =
        CrudApiScaffold::get_all::<CombatantEntity, CombatantRecord>(db).await;

    return Json(Vec::from_iter(
        all_combatants
            .iter()
            .map(|record: &CombatantRecord| CombatantContract::from(record)),
    ));
}

#[get("/<id>")]
pub async fn get_by_id(db: &State<Surreal<Client>>, id: &str) -> ApiResponse {
    return CrudApiScaffold::get_by_id::<CombatantEntity, CombatantRecord, CombatantContract>(
        db,
        id,
        |record: CombatantRecord| CombatantContract::from(&record),
    )
    .await;
}

#[delete("/<id>")]
pub async fn delete(db: &State<Surreal<Client>>, id: &str) -> ApiResponse {
    return CrudApiScaffold::delete::<CombatantEntity, CombatantRecord, CombatantContract>(
        db,
        id,
        |record: CombatantRecord| CombatantContract::from(&record),
    )
    .await;
}

#[post("/", format = "json", data = "<combatant_post_data>")]
pub async fn create_new(
    combatant_post_data: Json<CombatantContract>,
    db: &State<Surreal<Client>>,
) -> ApiResponse {
    let entity: CombatantEntity = CombatantEntity::from(combatant_post_data);
    return CrudApiScaffold::create_new(db, entity, |record: CombatantRecord| {
        CombatantContract::from(&record)
    })
    .await;
}

#[put("/<id>", format = "json", data = "<post_data>")]
pub async fn update(
    id: &str,
    post_data: Json<CombatantContract>,
    db: &State<Surreal<Client>>,
) -> ApiResponse {
    let entity: CombatantEntity = CombatantEntity::from(post_data);
    return CrudApiScaffold::update(db, id, entity, |record: CombatantRecord| {
        CombatantContract::from(&record)
    })
    .await;
}

//#[post("/<id>", format = "json", data = "<combatant_post_data>")]
//pub async fn update(
//    id: &str,
//    combatant_post_data: Json<CreateCombatantContract>,
//    db: &State<Surreal<Client>>) -> ApiResponse {
//        let updated_combatant: Result<CombatantRecord, surrealdb::Error> = CombatantEntity::update(db, id, combatant_post_data).await;//
//        match updated_combatant{
//            Ok(c) => ApiResponse{
//                json: serde_json::to_string(&c).unwrap(),
//                status: Status::Ok
//            },
//            Err(e) => ApiResponse {
//                json: e.to_string(),
//                status: Status::BadRequest,
//            },
//        }
//    }
