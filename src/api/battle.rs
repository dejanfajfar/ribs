use rocket::{
    http::Status,
    serde::json::{self, Json},
    State,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    engine::{
        battle_actions::BattleAction,
        battle_engine::BattleEngine,
        battle_result::BattleResult,
        battlefield::BattlefieldData,
        combatant::Combatant,
        err::{self, Error},
        map::Map,
    },
    types::point::Point,
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

#[derive(Serialize)]
pub struct BattleResultContract {
    pub combatants: Vec<CombatantContract>,
    pub map: MapContract,
    pub actions: Vec<BattleActionContract>,
    pub round_number: u32,
}

#[derive(Serialize)]
pub struct MapContract {
    width: u8,
    height: u8,
    pois: Vec<PoiContract>,
}

#[derive(Serialize)]
pub struct PoiContract {
    location: PointContract,
    name: String,
}

#[derive(Serialize)]
pub struct PointContract {
    x: u8,
    y: u8,
}

#[derive(Serialize)]
pub enum BattleActionContract {
    Move(BattleActionMoveContract),
    Attack(BattleActionAttackContract),
}

#[derive(Serialize)]
pub struct BattleActionMoveContract {
    round: u32,
    combatant: String,
    start: PointContract,
    end: PointContract,
    path: Vec<PointContract>,
}

#[derive(Serialize)]
pub struct BattleActionAttackContract {
    round: u32,
    attacker: String,
    attacked: String,
    dmg: u16,
    remaining_hp: u16
}

impl From<Point> for PointContract {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Map> for MapContract {
    fn from(value: Map) -> Self {
        Self {
            width: value.get_width(),
            height: value.get_height(),
            pois: value
                .get_pois()
                .iter()
                .map(|poi| PoiContract::from(poi))
                .collect(),
        }
    }
}

impl From<&(Point, String)> for PoiContract {
    fn from(value: &(Point, String)) -> Self {
        Self {
            location: PointContract::from(value.0),
            name: value.1.clone(),
        }
    }
}

impl From<BattleResult> for BattleResultContract {
    fn from(value: BattleResult) -> Self {
        Self {
            combatants: value
                .combatants
                .iter()
                .map(|c| CombatantContract::from(c))
                .collect(),
            map: MapContract::from(value.map),
            actions: value
                .actions
                .iter()
                .map(|action| BattleActionContract::from(action))
                .collect(),
            round_number: value.round_number,
        }
    }
}

impl From<&Combatant> for CombatantContract {
    fn from(value: &Combatant) -> Self {
        Self {
            name: value.name.clone(),
            id: None,
            hp: value.hp,
            dmg: value.dmg,
        }
    }
}

impl From<&BattleAction> for BattleActionContract {
    fn from(value: &BattleAction) -> Self {
        match value {
            BattleAction::Move(r_num, combatant, results) => {
                BattleActionContract::Move(BattleActionMoveContract {
                    round: r_num.clone(),
                    combatant: combatant.clone(),
                    start: PointContract::from(results.start),
                    end: PointContract::from(results.last_position),
                    path: results
                        .steps
                        .iter()
                        .map(|p| PointContract::from(p.clone()))
                        .collect(),
                })
            }
            BattleAction::Attack(r_num, action) => {
                BattleActionContract::Attack(BattleActionAttackContract {
                    round: r_num.clone(),
                    attacker: action.assailant.name.clone(),
                    attacked: action.victim.name.clone(),
                    dmg: action.assailant.dmg,
                    remaining_hp: action.victim.hp
                })
            }
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
                        Ok(result) => {
                            let serialization_result =
                                serde_json::to_string(&BattleResultContract::from(result));

                            match serialization_result {
                                Ok(json) => ApiResponse {
                                    json: json,
                                    status: Status::Ok,
                                },
                                Err(_) => ApiResponse::empty(Status::InternalServerError),
                            }
                        }
                        Err(error) => ApiResponse::from(error),
                    }
                }
                Err(_) => ApiResponse::empty(Status::InternalServerError),
            }
        }
        Err(_) => ApiResponse::empty(Status::BadRequest),
    }
}
