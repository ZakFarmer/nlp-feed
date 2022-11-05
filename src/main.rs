mod api;
mod external;
mod models;
mod repositories;
mod requests;
mod tasks;
mod utility;

#[macro_use]
extern crate log;

#[macro_use]
extern crate rocket;

use api::{
    healthcheck::healthcheck,
    post::{create_post, get_all_posts, get_post},
    utility::populate_posts,
};
use repositories::mongo::MongoRepository;
use rocket::Config;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    let db = MongoRepository::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![healthcheck])
        .mount(
            "/api",
            routes![get_post, get_all_posts, create_post, populate_posts],
        )
}
