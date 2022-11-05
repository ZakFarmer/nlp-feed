use std::fmt::Error;

use rocket::{http::Status, serde::json::Json, State};

use crate::{models::post::Post, repositories::mongo::MongoRepository, tasks::populate::populate};

#[post("/utility/populate")]
pub async fn populate_posts(db: &State<MongoRepository>) -> Result<Json<bool>, Status> {
    let result = populate(db).await;

    match result {
        Ok(val) => Ok(val),
        Err(_) => Err(Status::InternalServerError),
    };

    Ok(Json(true))
}
