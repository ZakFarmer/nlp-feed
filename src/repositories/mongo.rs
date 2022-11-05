use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    sync::{Client, Collection, Cursor},
};

use crate::models::post::Post;

pub struct MongoRepository {
    collection: Collection<Post>,
}

impl MongoRepository {
    pub fn init() -> Self {
        dotenv().ok();

        // Attempt to read MongoDB connection URI from .env
        let uri = match env::var("MONGODB_URI") {
            Ok(var) => var.to_string(),
            Err(_) => format!("Error loading MongoDB URI env variable."),
        };

        // Initialise database client
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("aiBlog");

        let collection: Collection<Post> = db.collection("Post");

        MongoRepository { collection }
    }

    pub fn create_post(&self, new_post: Post) -> Result<InsertOneResult, Error> {
        // Prepare the post for insertion to database
        let new_document = Post {
            id: None,
            title: new_post.title,
            content: new_post.content,
        };

        // Add the post to the database
        let post = self
            .collection
            .insert_one(new_document, None)
            .ok()
            .expect("Couldn't create new post.");

        Ok(post)
    }

    pub fn get_post(&self, id: &String) -> Result<Post, Error> {
        // Parse the object ID from the string input
        let object_id = ObjectId::parse_str(id)?;

        // Configure filter and specify ID
        let filter = doc! {"_id": object_id};

        // Query the database for the post
        let result = self
            .collection
            .find_one(filter, None)
            .ok()
            .expect("Couldn't get post.")
            .unwrap(); // Okay to unwrap here as we catch any exceptions above

        Ok(result)
    }

    pub fn get_all_posts(&self) -> Result<Vec<Post>, Error> {
        let cursors = self
            .collection
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
            .collection
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
                "title": new_post.title,
                "content": new_post.content,
            },
        };

        let post = self
            .collection
            .update_one(filter, new_document, None)
            .ok()
            .expect("Couldn't update post.");

        Ok(post)
    }
}
