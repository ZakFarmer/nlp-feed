use crate::{
    models::{avatar::Avatar, post::Post},
    repositories::mongo::MongoRepository,
};
use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAvatarParams {
    first_name: String,
    last_name: String,
    description: String,
    location: String,
    keywords: String,
    repetition_penalty: f32,
    temperature: f32,
}

#[get("/avatar/<path>")]
pub fn get_avatar(db: &State<MongoRepository>, path: String) -> Result<Json<Avatar>, Status> {
    let avatar_id = path;

    if avatar_id.is_empty() {
        return Err(Status::BadRequest);
    }

    let status = db.get_avatar(&avatar_id);

    match status {
        Ok(avatar) => Ok(Json(avatar)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/avatars")]
pub fn get_all_avatars(db: &State<MongoRepository>) -> Result<Json<Vec<Avatar>>, Status> {
    let avatars = db.get_all_avatars();

    match avatars {
        Ok(avatars) => Ok(Json(avatars)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/avatar", data = "<avatar_params>")]
pub fn create_avatar(
    db: &State<MongoRepository>,
    avatar_params: Json<CreateAvatarParams>,
) -> Result<Json<InsertOneResult>, Status> {
    let data: Avatar = Avatar {
        id: None,
        first_name: avatar_params.first_name.to_owned(),
        last_name: avatar_params.last_name.to_owned(),
        description: avatar_params.description.to_owned(),
        location: avatar_params.location.to_owned(),
        keywords: avatar_params.keywords.to_owned(),
        repetition_penalty: avatar_params.repetition_penalty.to_owned(),
        temperature: avatar_params.temperature.to_owned(),
    };

    let status: Result<InsertOneResult, Error> = db.create_avatar(data);

    match status {
        Ok(avatar) => Ok(Json(avatar)),
        Err(_) => Err(Status::InternalServerError),
    }
}
