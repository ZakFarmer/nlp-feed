use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
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
            Err(_) => format!("Error loading MongoDB URI env variable"),
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
            link_to_article: new_post.link_to_article,
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
        let object_id = match ObjectId::parse_str(id) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        let filter = doc! {"_id": object_id};

        // Query the database for the post
        let result = self
            .collection
            .find_one(filter, None)
            .ok()
            .expect("Couldn't get post.");

        Ok(result.unwrap()) // Okay to unwrap here as we catch any exceptions above
    }
}
