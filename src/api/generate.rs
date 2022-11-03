use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    models::post::Post, repositories::mongo::MongoRepository, requests::post::PostRequest,
};

#[post("/generate-post", data = "<post_params>")]
pub fn generate_post(
    db: &State<MongoRepository>,
    post_params: Json<PostRequest>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Post {
        id: None,
        title: "Not populated yet.".to_string(),
        content: "Not populated yet.".to_string(),
        link_to_article: post_params.link_to_article.to_owned(),
    };

    let status = db.create_post(data);

    match status {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}
