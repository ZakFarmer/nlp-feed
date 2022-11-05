use std::{collections::HashMap, env, fmt::Error};

use dotenv::dotenv;
use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Client, StatusCode,
};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::article::Articles;

pub struct NewsCatcherClient {
    api_token: String,
}

impl NewsCatcherClient {
    pub fn init() -> Self {
        dotenv().ok();

        let api_token = match env::var("NEWS_API_KEY") {
            Ok(var) => var.to_string(),
            Err(_) => format!("Error loading News API Key env variable."),
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
