use crate::{
    models::{avatar::Avatar, post::Post},
    repositories::mongo::MongoRepository,
};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{
    http::Status,
    response::stream::{Event, EventStream},
    serde::json::Json,
    tokio::sync::broadcast::{error::RecvError, Sender},
    tokio::{select, sync::broadcast::Receiver},
    Shutdown, State,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePostParams {
    pub avatar_id: ObjectId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub avatar: Avatar,
    pub content: String,
    pub date_published: String,
}

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

#[get("/new-posts")]
pub fn stream_new_posts(queue: &State<Sender<NewPost>>, mut halt: Shutdown) -> EventStream![] {
    let mut rx: Receiver<NewPost> = queue.subscribe();

    EventStream! {
        loop {
            let message = select! {
                message = rx.recv() => match message {
                    Ok(message) => message,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut halt => break,
            };

            yield Event::json(&message);
        }
    }
}

#[post("/post", data = "<post_params>")]
pub async fn create_post(
    db: &State<MongoRepository>,
    queue: &State<Sender<NewPost>>,
    post_params: Json<CreatePostParams>,
) -> Result<Json<InsertOneResult>, Status> {
    let avatar = db.get_avatar(&post_params.avatar_id.to_string()).unwrap();

    let data = Post {
        id: None,
        avatar: avatar.clone(),
        content: "".to_string(),
        date_published: "".to_string(),
    };

    let status = db.create_post(data, avatar, queue);

    match status {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}
