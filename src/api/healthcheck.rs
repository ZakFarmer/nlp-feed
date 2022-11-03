use rocket::{http::Status, serde::json::Json};

#[get("/healthcheck")]
pub fn healthcheck() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Healthcheck OK")))
}
