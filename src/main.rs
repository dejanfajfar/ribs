use std::net::Ipv4Addr;

use rocket::Config;
use storage::middleware::DbMiddleware;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod api;
mod engine;
mod storage;
mod types;

#[launch]
fn rocket() -> _ {
    env_logger::init();
    rocket::build()
        .mount("/", routes![api::index])
        .mount(
            "/battlefields",
            routes![
                api::battlefield::get_all,
                api::battlefield::create_new,
                api::battlefield::update,
                api::battlefield::delete,
                api::battlefield::get_by_id
            ],
        )
        .mount(
            "/combatants",
            routes![
                api::combatant::get_all,
                api::combatant::create_new,
                api::combatant::update,
                api::combatant::delete,
                api::combatant::get_by_id
            ],
        )
        .attach(DbMiddleware)
        .configure(Config {
            port: 7777,
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Config::debug_default()
        })
}
