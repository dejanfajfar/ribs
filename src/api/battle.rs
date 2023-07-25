use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::engine::{
    battle_engine::BattleEngine, battlefield::BattlefieldData, combatant::Combatant, err::Error,
};

use super::{battlefield::BattleFieldContract, combatant::CombatantContract, ApiResponse};

#[derive(Serialize, Deserialize)]
pub struct CreateBattleContract {
    battlefield: BattleFieldContract,
}

impl TryFrom<Json<CreateBattleContract>> for BattlefieldData {
    type Error = crate::engine::err::Error;

    fn try_from(value: Json<CreateBattleContract>) -> Result<Self, Self::Error> {
        let mut battlefield = BattlefieldData {
            battlefield_height: value.battlefield.height,
            battlefield_width: value.battlefield.width,
            combatants: value
                .battlefield
                .combatants
                .iter()
                .map(|c| Combatant::from(c))
                .collect(),
        };

        return Ok(battlefield);
    }
}

impl From<&CombatantContract> for Combatant {
    fn from(value: &CombatantContract) -> Self {
        Combatant {
            name: value.name.clone(),
            hp: value.hp,
            dmg: value.dmg,
        }
    }
}

#[post("/", format = "json", data = "<post_data>")]
pub async fn start_new_battle(
    post_data: Json<CreateBattleContract>,
    db: &State<Surreal<Client>>,
) -> ApiResponse {
    let battlefield = BattlefieldData::try_from(post_data);

    match battlefield {
        Ok(b) => {
            let engine = BattleEngine::new(b);

            match engine {
                Ok(mut e) => {
                    let battle_results = e.start_battle();

                    match battle_results {
                        Ok(result) => todo!(),
                        Err(error) => todo!()
                    }
                },
                Err(_) => ApiResponse::empty(Status::InternalServerError)
            }
            

            
        }
        Err(_) => ApiResponse::empty(Status::BadRequest),
    }
}
