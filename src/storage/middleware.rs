use std::env;

use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    Build, Rocket,
};
use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::{Ws, Client},
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

impl DbMiddleware {

    /// Returns an active connection to the database
    /// 
    /// The connection data is retrieved from the DbConfig default value 
    async fn connect(&self) -> Result<Surreal<Client>, surrealdb::Error> {
        let db_config: DbConfig = DbConfig::default();
        let db = Surreal::new::<Ws>(db_config.address.clone()).await?;

        db.signin(Root {
            password: &db_config.pass,
            username: &db_config.user,
        })
        .await?;

        db.use_ns(db_config.namespace)
        .use_db(db_config.database)
        .await?;

        return Ok(db);
    }
}

#[rocket::async_trait]
impl Fairing for DbMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Database Middleware",
            kind: Kind::Ignite,
        }
    }

    /// Enrolls the db connection state management at application "ignite"
    /// 
    /// EXPLANATION:
    /// We can do this because we are using a standing WS (WebSocket) connection to the database
    /// Because of that we only need to open a connection at application start and then only reuse it
    /// 
    /// NOTE:
    /// In the case that the connection to the database is interrupted we do not reestablish automatically!
    /// The application has to be restarted!
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let db = self.connect().await;
        match db {
            Ok(db_connection) => Ok(rocket.manage(db_connection)),
            Err(e) => panic!(
                "Could not connect to {} with reason: {}",
                DbConfig::default().address,
                e.to_string()
            ),
        }
    }
}
