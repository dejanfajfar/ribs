#[macro_use] extern crate rocket;

mod engine;
mod api;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![api::index])
    .mount("/battlefields", routes![api::battlefield::get_battlefield])
}
