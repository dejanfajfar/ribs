use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    api::ApiResponse,
    storage::{battlefields::*, Record, combatants::CombatantEntity},
};

use super::{CrudApiScaffold, combatant::CombatantContract};

#[derive(Serialize, Deserialize)]
pub struct BattleFieldContract {
    pub height: u8,
    pub width: u8,
    pub id: Option<String>,
    pub combatants: Vec<CombatantContract>
}

impl From<Json<BattleFieldContract>> for BattleFieldEntity {
    fn from(value: Json<BattleFieldContract>) -> Self {
        BattleFieldEntity {
            height: value.height,
            width: value.width,
            combatants: value.combatants.iter().map(|c| CombatantEntity::from(Json(c.clone()))).collect()
        }
    }
}

impl From<&BattleFieldRecord> for BattleFieldContract {
    fn from(value: &BattleFieldRecord) -> Self {
        BattleFieldContract {
            height: value.height,
            width: value.width,
            id: Some(value.get_id()),
            combatants: value.combatants.iter().map(|c| CombatantContract::from(c)).collect()
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
