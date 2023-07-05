pub mod armor_store;
pub mod weapon_store;
pub mod middleware;
pub mod combatants;

use rocket::data;
use std::env;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::{Database, Root};
use surrealdb::Surreal;

pub async fn connect_to_db<'a>() -> surrealdb::Result<Surreal<Client>> {
    let address = env_or_default("db_address", "127.0.0.1:8000");
    let username = env_or_default("db_username", "root");
    let password = env_or_default("db_password", "root");
    let namespace = env_or_default("db_namespace", "development");
    let database = env_or_default("db_name", "ribs");

    let db = Surreal::new::<Ws>(address).await?;

    db.signin(Root {
        password: &password,
        username: &username,
    })
    .await?;

    db.use_ns(&namespace).use_db(&database).await?;

    return Ok(db);
}

fn env_or_default<'a>(key: &'a str, default: &'a str) -> String {
    let env_opt_val = env::var(key);

    match env_opt_val {
        Ok(val) => val,
        Err(_) => default.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
