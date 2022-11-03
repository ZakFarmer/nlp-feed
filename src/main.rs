mod api;
mod models;
mod repositories;
mod requests;

#[macro_use]
extern crate rocket;

use api::{
    generate::generate_post,
    healthcheck::healthcheck,
    post::{get_all_posts, get_post},
};
use repositories::mongo::MongoRepository;

#[launch]
fn rocket() -> _ {
    let db = MongoRepository::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![healthcheck])
        .mount("/api", routes![get_post, get_all_posts, generate_post])
}
