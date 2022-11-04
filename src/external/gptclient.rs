use std::{collections::HashMap, env, fmt::Error};

use dotenv::dotenv;

pub struct GptClient {
    api_token: String,
    api_url: String,
}

impl GptClient {
    pub fn init() -> Self {
        dotenv().ok();

        let api_token = match env::var("GPT_API_TOKEN") {
            Ok(var) => var.to_string(),
            Err(_) => format!("Error loading GPT API Token env variable."),
        };

        let api_url = match env::var("GPT_API_URL") {
            Ok(var) => var.to_string(),
            Err(_) => format!("Error loading GPT API base URL env variable."),
        };

        GptClient { api_token, api_url }
    }

    pub async fn query(&self, prompt: String) -> Result<String, Error> {
        let mut payload = HashMap::new();
        payload.insert("inputs", prompt);

        let client = reqwest::Client::new();

        let res = client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&payload)
            .send()
            .await
            .expect("Couldn't query GPT API.");

        let res_text = res.text().await.unwrap();

        Ok(res_text)
    }
}
