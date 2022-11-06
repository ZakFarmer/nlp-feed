



use rand::seq::SliceRandom;
use rocket::{tokio::sync::broadcast::Sender, State};

use crate::{
    api::post::NewPost,
    exceptions::gpt::GPTException,
    external::gptclient::GptClient,
    models::{avatar::Avatar, post::Post},
    repositories::mongo::MongoRepository,
    utility::prompt::{import_prompt, Prompt},
};

pub async fn populate(
    db: &State<MongoRepository>,
    avatar: Avatar,
    queue: &State<Sender<NewPost>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Initialise GPT client
    let gpt_client = GptClient::init();

    let mut keywords_string: String = avatar.keywords.clone();

    keywords_string.retain(|chr| !chr.is_whitespace());

    let keywords = keywords_string.split(',');

    let keywords_vec: Vec<&str> = keywords.collect();

    let random_keyword = keywords_vec
        .choose(&mut rand::thread_rng())
        .expect("Couldn't get random keyword.");

    // Import prompt
    let prompt_content = import_prompt(Prompt::Blog, String::from(*random_keyword)).unwrap();

    info!(
        "[Avatar {}]: Prompting GPT with keyword: {}",
        avatar.id.unwrap(),
        random_keyword.to_string(),
    );

    // Query the model with the prompt
    let mut gpt_response = gpt_client.query(prompt_content).await?;

    // Replace some extra characters that GPT comes back with sometimes
    gpt_response = gpt_response.replace(&['(', ')', '\"', '\''][..], "");

    // Trim whitespace
    gpt_response = gpt_response.trim().to_string();

    if gpt_response.len() > 200 {
        // If the response is longer than 200 it's probably because the model has
        // started talking gibberish, so we discard this one
        return Err(Box::new(GPTException::ResponseTooLongException));
    }

    let new_post = Post {
        id: None,
        content: gpt_response,
        avatar: avatar.clone(),
        date_published: "".to_string(),
    };

    // Add post to the database
    db.create_post(new_post, avatar, queue)?;

    Ok(true)
}
