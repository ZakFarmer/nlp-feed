use std::{env, time::Duration};

use async_std::task;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use rocket::State;
use serde_json::Value;

use crate::{
    external::{gptclient::GptClient, newscatcherclient::NewsCatcherClient},
    models::{article::Articles, post::Post},
    repositories::mongo::MongoRepository,
    utility::{
        prompt::{import_prompt, Prompt},
        string::truncate,
    },
};

pub async fn populate(db: &State<MongoRepository>) -> Result<bool, Box<dyn std::error::Error>> {
    // Initialise GPT client
    let gpt_client = GptClient::init();

    // Initalise index to use for iterating through articles
    let mut index: usize = 0;

    // Iterate over the posts marked as unpopulated (i.e. not processed by the model yet)
    for index in 1..15 {
        // Import prompt
        let prompt_content = import_prompt(Prompt::Blog, String::from("usa")).unwrap();

        info!("Prompting GPT with: {}", prompt_content);

        // Query the model with the prompt
        let gpt_response = gpt_client.query(prompt_content).await?;

        let new_post = Post {
            id: None,
            title: format!("Generated post #{}", index),
            content: gpt_response.to_string(),
        };

        // Add post to the database
        db.create_post(new_post)?;

        task::sleep(Duration::from_secs(1)).await;
    }

    Ok(true)
}
