use rocket::Config;

#[macro_use] extern crate rocket;

mod engine;
mod api;
mod storage;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![api::index])
    .mount("/battlefields", routes![api::battlefield::get_battlefield])
    .mount("/armors", routes![api::armor::get_all, api::armor::post_armor])
    .configure(Config {
        port: 7777,
        ..Config::debug_default()
    })
}
