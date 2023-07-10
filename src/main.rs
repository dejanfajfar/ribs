use std::net::Ipv4Addr;

use rocket::Config;
use storage::middleware::DbMiddleware;

#[macro_use]
extern crate rocket;

mod api;
mod engine;
mod storage;
mod types;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![api::index])
        .mount(
            "/battlefields",
            routes![
                api::battlefield::get_all,
                api::battlefield::create_new,
                api::battlefield::update
            ],
        )
        .mount(
            "/armors",
            routes![
                api::armor::get_all,
                api::armor::create_armor,
                api::armor::update_armor
            ],
        )
        .mount(
            "/combatants",
            routes![
                api::combatant::get_all,
                api::combatant::create_new,
                api::combatant::update
            ],
        )
        .attach(DbMiddleware)
        .configure(Config {
            port: 7777,
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Config::debug_default()
        })
}
