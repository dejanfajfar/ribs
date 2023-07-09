use std::{default, env};

use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    Build, Rocket,
};
use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Deserialize)]
struct DbConfig {
    namespace: String,
    database: String,
    user: String,
    pass: String,
    address: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        let address = env_or_default("db_address", "127.0.0.1:8000");
        let user = env_or_default("db_username", "root");
        let pass = env_or_default("db_password", "root");
        let namespace = env_or_default("db_namespace", "development");
        let database = env_or_default("db_name", "ribs");
        Self {
            namespace,
            database,
            user,
            pass,
            address,
        }
    }
}

fn env_or_default<'a>(key: &'a str, default: &'a str) -> String {
    let env_opt_val = env::var(key);

    match env_opt_val {
        Ok(val) => val,
        Err(_) => default.to_owned(),
    }
}

pub struct DbMiddleware;

#[rocket::async_trait]
impl Fairing for DbMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Database Middleware",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let figment = rocket.figment().clone();

        let db_config: DbConfig = DbConfig::default();

        let db: Surreal<Client> = Surreal::new::<Ws>(db_config.address).await.unwrap();

        db.signin(Root {
            password: &db_config.pass,
            username: &db_config.user,
        })
        .await
        .unwrap();

        db.use_ns(db_config.namespace)
            .use_db(db_config.database)
            .await
            .unwrap();

        Ok(rocket.manage(db))
    }
}
