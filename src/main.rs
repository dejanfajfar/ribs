use std::net::Ipv4Addr;

use configuration::Config as c;
use rocket::Config;
use storage::middleware::DbMiddleware;

#[macro_use] extern crate rocket;

mod configuration;
mod engine;
mod api;
mod storage;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![api::index])
    .mount("/battlefields", routes![api::battlefield::get_battlefield])
    .mount("/armors", routes![api::armor::get_all, api::armor::create_armor, api::armor::update_armor]).attach(DbMiddleware)
    .configure(Config {
        port: 7777,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::debug_default()
    })
}
