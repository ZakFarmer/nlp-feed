use crate::{
    models::post::Post, repositories::mongo::MongoRepository, requests::post::PostRequest,
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[get("/post/<path>")]
pub fn get_post(db: &State<MongoRepository>, path: String) -> Result<Json<Post>, Status> {
    let post_id = path;

    if post_id.is_empty() {
        return Err(Status::BadRequest);
    }

    let status = db.get_post(&post_id);

    match status {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/posts")]
pub fn get_all_posts(db: &State<MongoRepository>) -> Result<Json<Vec<Post>>, Status> {
    let posts = db.get_all_posts();

    match posts {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(Status::InternalServerError),
    }
}
