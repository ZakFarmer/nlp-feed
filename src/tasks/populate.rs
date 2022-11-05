use std::time::Duration;

use async_std::task;

use rocket::State;

use crate::{
    external::gptclient::GptClient,
    models::post::Post,
    repositories::mongo::MongoRepository,
    utility::prompt::{import_prompt, Prompt},
};

pub async fn populate(db: &State<MongoRepository>) -> Result<bool, Box<dyn std::error::Error>> {
    // Initialise GPT client
    let gpt_client = GptClient::init();

    // Initalise index to use for iterating through articles
    let _index: usize = 0;

    // Iterate over the posts marked as unpopulated (i.e. not processed by the model yet)
    for index in 1..50 {
        // Import prompt
        let prompt_content = import_prompt(Prompt::Blog, String::from("usa")).unwrap();

        info!("Prompting GPT with: {}", prompt_content);

        // Query the model with the prompt
        let mut gpt_response = gpt_client.query(prompt_content).await?;

        // Replace some extra characters that GPT comes back with sometimes
        gpt_response = gpt_response.replace(&['(', ')', '\"', '\''][..], "");

        // Trim whitespace
        gpt_response = gpt_response.trim().to_string();

        if (gpt_response.len() > 200) {
            // If the response is longer than 200 it's probably because the model has
            // started talking gibberish, so we discard this one and don't add it to the DB
            continue;
        }

        let new_post = Post {
            id: None,
            title: format!("Generated post #{}", index),
            content: gpt_response.to_string(),
        };

        // Add post to the database
        db.create_post(new_post)?;

        // Sleep for a second to avoid hitting concurrent requests limit
        task::sleep(Duration::from_secs(1)).await;
    }

    Ok(true)
}
