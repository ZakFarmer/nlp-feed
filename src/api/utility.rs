use rocket::{http::Status, serde::json::Json, tokio::sync::broadcast::Sender, State};
use serde::Deserialize;

use crate::{models::post::Post, repositories::mongo::MongoRepository, tasks::populate::populate};

use super::post::NewPost;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PopulatePostsParams {
    pub avatar_id: String,
}

#[post("/utility/populate", data = "<populate_params>")]
pub async fn populate_posts(
    db: &State<MongoRepository>,
    populate_params: Json<PopulatePostsParams>,
    state: &State<Sender<NewPost>>,
) -> Result<Json<bool>, Status> {
    let avatar = db
        .get_avatar(&populate_params.avatar_id)
        .expect("Couldn't get avatar.");

    let result = populate(db, avatar, state).await;

    match result {
        Ok(val) => Ok(val),
        Err(_) => Err(Status::InternalServerError),
    };

    Ok(Json(true))
}
