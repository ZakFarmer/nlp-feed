mod api;
mod models;
mod repositories;
mod requests;

#[macro_use]
extern crate rocket;

use api::{healthcheck::healthcheck, post::generate_post};
use repositories::mongo::MongoRepository;

#[launch]
fn rocket() -> _ {
    let db = MongoRepository::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![healthcheck])
        .mount("/api", routes![generate_post])
}
