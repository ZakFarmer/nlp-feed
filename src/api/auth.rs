extern crate bcrypt;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{models::user::User, repositories::mongo::MongoRepository};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterParams {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginParams {
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthResponse {
    access_token: String,
    user: User,
}

#[post("/auth/register", format = "json", data = "<register_params>")]
pub fn register(
    db: &State<MongoRepository>,
    register_params: Json<RegisterParams>,
) -> Result<Json<AuthResponse>, Status> {
    let password = register_params.password.to_owned();

    let hashed_password = hash(password, DEFAULT_COST);

    let user_data: User = User {
        id: None,
        username: register_params.username.to_owned(),
        email: register_params.email.to_owned(),
        email_verified_at: Some("".to_string()),
        hashed_password: hashed_password.unwrap(),
    };

    let status = db.create_user(data.clone());

    match status {
        Ok(_) => Ok(Json(data)),
        Err(_) => return Err(Status::InternalServerError),
    }
}

#[post("/auth/login", format = "json", data = "<login_params>")]
pub fn login(
    db: &State<MongoRepository>,
    login_params: Json<LoginParams>,
) -> Result<Json<LoginResponse>, Status> {
}
