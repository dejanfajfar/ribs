use crate::storage::combatants::*;
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use super::ApiResponse;

#[derive(Deserialize)]
pub struct CreateCombatantContract {
    pub name: String,
    pub hp: u16,
    pub dmg: u16,
}

#[derive(Deserialize, Serialize)]
pub struct CombatantContract {
    pub name: String,
    pub id: String,
    pub hp: u16,
    pub dmg: u16,
}

impl From<CombatantRecord> for CombatantContract {
    fn from(value: CombatantRecord) -> Self {
        CombatantContract {
            name: value.name.clone(),
            id: value.id.id.to_string(),
            hp: value.hit_points,
            dmg: value.damage_rating,
        }
    }
}

impl From<Json<CreateCombatantContract>> for CombatantEntity {
    fn from(value: Json<CreateCombatantContract>) -> Self {
        CombatantEntity {
            name: value.name.clone(),
            damage_rating: value.dmg,
            hit_points: value.hp,
        }
    }
}

#[get("/")]
pub async fn get_all(db: &State<Surreal<Client>>) -> Json<Vec<CombatantContract>> {
    let all_combatants: Result<Vec<CombatantRecord>, surrealdb::Error> =
        CombatantEntity::get_all(db).await;

    match all_combatants {
        Ok(c) => {
            let mut ret_val = vec![];
            for cc in c {
                ret_val.push(CombatantContract::from(cc));
            }

            return Json(ret_val);
        }
        Err(_) => Json(vec![]),
    }
}

#[post("/", format = "json", data = "<combatant_post_data>")]
pub async fn create_new(
    combatant_post_data: Json<CreateCombatantContract>,
    db: &State<Surreal<Client>>,
) -> ApiResponse {
    let combatants: Result<CombatantRecord, surrealdb::Error> =
        CombatantEntity::crate_new(db, CombatantEntity::from(combatant_post_data)).await;

    match combatants {
        Ok(c) => ApiResponse {
            json: serde_json::to_string(&c).unwrap(),
            status: Status::Ok,
        },
        Err(e) => ApiResponse {
            json: e.to_string(),
            status: Status::BadRequest,
        },
    }
}
