use rocket::{http::Status, serde::json::Json, tokio::sync::broadcast::Sender, State};
use serde::Deserialize;

use crate::{repositories::mongo::MongoRepository, tasks::populate::populate};

use super::post::NewPost;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PopulatePostsParams {
    pub avatar_id: String,
}

#[post("/utility/populate")]
pub async fn populate_posts(
    db: &State<MongoRepository>,
    queue: &State<Sender<NewPost>>,
) -> Result<Json<bool>, Status> {
    let avatars = db
        .get_all_avatars()
        .expect("Couldn't retrieve all avatars.");

    let result = populate(db, avatars, queue).await;

    match result {
        Ok(value) => Ok(Json(value)),
        Err(_) => return Err(Status::InternalServerError),
    }
}
