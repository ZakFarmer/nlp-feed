mod api;
mod exceptions;
mod external;
mod fairings;
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
    avatar::{create_avatar, get_all_avatars, get_avatar},
    healthcheck::healthcheck,
    post::{create_post, get_all_posts, get_post, stream_new_posts, NewPost},
    utility::populate_posts,
};
use fairings::cors::CORS;
use models::post::Post;
use repositories::mongo::MongoRepository;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};

#[launch]
fn rocket() -> _ {
    env_logger::init();

    let db = MongoRepository::init();

    rocket::build()
        .manage(channel::<NewPost>(1024).0) // Tell Rocket to manage the new post channel
        .manage(db)
        .attach(CORS)
        .mount("/", routes![healthcheck])
        .mount(
            "/api",
            routes![
                get_post,
                get_all_posts,
                create_post,
                populate_posts,
                stream_new_posts,
                create_avatar,
                get_avatar,
                get_all_avatars
            ],
        )
}
