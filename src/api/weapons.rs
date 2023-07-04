use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::types::skills::*;

#[derive(Deserialize)]
pub struct CreateWeapon<'r>{
    pub name: &'r str,
    pub dmg: u16,
    pub min_stats: Skills
}


#[get("/")]
pub async get_all(db: &State<Surreal<Client>>) -> Json<Vec<>> {
    
}