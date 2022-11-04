use std::fmt::Error;

use rocket::{http::Status, serde::json::Json, State};

use crate::{models::post::Post, repositories::mongo::MongoRepository, tasks::generate::populate};

#[post("/utility/populate")]
pub async fn populate_posts(db: &State<MongoRepository>) -> Result<Json<String>, Status> {
    populate(db).await;

    Ok(Json(String::from("Spawned post population process.")))
}
