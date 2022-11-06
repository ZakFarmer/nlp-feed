use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId, Document},
    results::{InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use rocket::{tokio::sync::broadcast::Sender, State};

use crate::{
    api::post::NewPost,
    models::{avatar::Avatar, post::Post},
};

pub struct MongoRepository {
    avatar_collection: Collection<Avatar>,
    post_collection: Collection<Post>,
}

impl MongoRepository {
    pub fn init() -> Self {
        dotenv().ok();

        // Attempt to read MongoDB connection URI from .env
        let uri = match env::var("MONGODB_URI") {
            Ok(var) => var,
            Err(_) => "Error loading MongoDB URI env variable.".to_string(),
        };

        // Initialise database client
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("aiBlog");

        let avatar_collection: Collection<Avatar> = db.collection("Avatar");
        let post_collection: Collection<Post> = db.collection("Post");

        MongoRepository {
            avatar_collection,
            post_collection,
        }
    }

    pub fn create_avatar(&self, new_avatar: Avatar) -> Result<InsertOneResult, Error> {
        let new_document: Avatar = Avatar {
            id: None,
            first_name: new_avatar.first_name,
            last_name: new_avatar.last_name,
            location: new_avatar.location,
            keywords: new_avatar.keywords,
        };

        let avatar = self
            .avatar_collection
            .insert_one(new_document, None)
            .ok()
            .expect("Couldn't create new avatar.");

        Ok(avatar)
    }

    pub fn create_post(
        &self,
        new_post: Post,
        avatar: Avatar,
        queue: &State<Sender<NewPost>>,
    ) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
        // Prepare the post for insertion to database
        let new_document: Post = Post {
            id: None,
            content: new_post.content,
            avatar: avatar.clone(),
            date_published: format!("{}", chrono::Utc::now().format("%d/%m/%Y %H:%M")),
        };

        // Add the post to the database
        let post = self
            .post_collection
            .insert_one(&new_document, None)
            .ok()
            .expect("Couldn't create new post.");

        // Broadcast that a new post has been created
        // Note: passing the state to the repository like this feels messy but
        // it was a lot quicker than doing it the right way :)
        let new_post: NewPost = NewPost {
            content: new_document.content,
            avatar: avatar,
            date_published: chrono::Utc::now().to_string(),
        };

        queue.send(new_post)?;

        Ok(post)
    }

    pub fn get_avatar(&self, id: &String) -> Result<Avatar, Error> {
        let object_id: ObjectId = ObjectId::parse_str(id)?;

        let filter: Document = doc! {"_id": object_id};

        let result = self
            .avatar_collection
            .find_one(filter, None)
            .ok()
            .expect("Couldn't get avatar.")
            .unwrap();

        Ok(result)
    }

    pub fn get_post(&self, id: &String) -> Result<Post, Error> {
        // Parse the object ID from the string input
        let object_id = ObjectId::parse_str(id)?;

        // Configure filter and specify ID
        let filter = doc! {"_id": object_id};

        // Query the database for the post
        let result = self
            .post_collection
            .find_one(filter, None)
            .ok()
            .expect("Couldn't get post.")
            .unwrap(); // Okay to unwrap here as we catch any exceptions above

        Ok(result)
    }

    pub fn get_all_avatars(&self) -> Result<Vec<Avatar>, Error> {
        let cursors = self
            .avatar_collection
            .find(None, None)
            .ok()
            .expect("Couldn't get all avatars.");

        let avatars = cursors.map(|document| document.unwrap()).collect();

        Ok(avatars)
    }

    pub fn get_all_posts(&self) -> Result<Vec<Post>, Error> {
        let cursors = self
            .post_collection
            .find(None, None)
            .ok()
            .expect("Couldn't get all posts.");

        let posts = cursors.map(|document| document.unwrap()).collect();

        Ok(posts)
    }

    pub fn get_unpopulated_posts(&self) -> Result<Vec<Post>, Error> {
        // Configure filter to get all posts where populated is false
        let filter = doc! {"populated": false};

        let cursors = self
            .post_collection
            .find(filter, None)
            .ok()
            .expect("Could get unpopulated posts.");

        let posts = cursors.map(|document| document.unwrap()).collect();

        Ok(posts)
    }

    pub fn update_post(&self, id: &String, new_post: Post) -> Result<UpdateResult, Error> {
        let object_id = ObjectId::parse_str(id)?;

        let filter = doc! {"_id": object_id};

        // Define updated document
        let new_document = doc! {
            "$set": {
                "id": new_post.id,
                "content": new_post.content,
            },
        };

        let post = self
            .post_collection
            .update_one(filter, new_document, None)
            .ok()
            .expect("Couldn't update post.");

        Ok(post)
    }
}
