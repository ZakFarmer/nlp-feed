use std::{env};

use dotenv::dotenv;
use reqwest::{
    header::{USER_AGENT},
    Client,
};


use serde_json::Value;



pub struct NewsCatcherClient {
    api_token: String,
}

impl NewsCatcherClient {
    pub fn init() -> Self {
        dotenv().ok();

        let api_token = match env::var("NEWS_API_KEY") {
            Ok(var) => var,
            Err(_) => "Error loading News API Key env variable.".to_string(),
        };

        NewsCatcherClient { api_token }
    }

    pub async fn get_latest_articles(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let url: &str = "https://api.newscatcherapi.com/v2/latest_headlines?sources=bbc.co.uk";

        /*
         *  Initialise reqwest HTTP client
         *  Note: this should probably be defined in the struct and lifetimed but as
         *  this won't be running often we can call it inline at the method level
         */
        let client = Client::new();

        let response = client
            .get(url)
            .header("x-api-key", &self.api_token)
            .header(USER_AGENT, "AI News Opinions API")
            .send()
            .await?;

        let response_text = response.text().await?;

        // Parses the response to a generalised serde Value (so that it's less likely to break if the schema changes)
        // Serde can also deserialise JSON in a strongly-typed way by specifying the chosen struct as the data type
        // e.g. let articles: Articles = serde_json::from_str(&response_text)?;
        let articles: Value = serde_json::from_str(&response_text)?;

        Ok(articles)
    }
}
