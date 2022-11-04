use std::env;

use chrono::{DateTime, Duration, Utc};
use dotenv::dotenv;
use newsapi::api::NewsAPIClient;
use rocket::State;

use crate::{external::gptclient::GptClient, repositories::mongo::MongoRepository};

pub async fn populate(db: &State<MongoRepository>) {
    // Initialise dotenv
    dotenv().ok();

    // Determine API token value from environment variable
    let api_token = match env::var("NEWS_API_TOKEN") {
        Ok(var) => var.to_string(),
        Err(_) => format!("Error loading News API Token env variable."),
    };

    // Get all posts currently unpopulated
    let posts = db.get_unpopulated_posts();

    // Duration of last 10 days
    let start_datetime: DateTime<Utc> = Utc::now() - Duration::days(10);
    let end_datetime: DateTime<Utc> = Utc::now();

    // Initialise News API client
    let news_client: NewsAPIClient = NewsAPIClient::new(api_token);

    // Iterate over the posts marked as unpopulated (i.e. not processed by the model yet)
    for post in posts {}
}
