use std::{collections::HashMap, env, error::Error};

use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{Map, Number, Value};

pub struct GptClient {
    api_token: String,
    api_url: String,
}

impl GptClient {
    pub fn init() -> Self {
        // Initalise dotenv
        dotenv().ok();

        // Initalise environment variables
        let api_token = match env::var("GPT_API_TOKEN") {
            Ok(var) => var,
            Err(_) => "Error loading GPT API Token env variable.".to_string(),
        };

        let api_url = match env::var("GPT_API_URL") {
            Ok(var) => var,
            Err(_) => "Error loading GPT API base URL env variable.".to_string(),
        };

        GptClient { api_token, api_url }
    }

    pub async fn query(
        &self,
        prompt: String,
        repetition_penalty: f32,
        temperature: f32,
    ) -> Result<String, Box<dyn Error>> {
        // Initialise payload for GPT request
        let mut payload = Map::new();
        payload.insert("text".to_string(), Value::String(prompt));
        payload.insert("length_no_input".to_string(), Value::Bool(true)); // Whether max_length should include input length or not
        payload.insert("end_sequence".to_string(), Value::String("###".to_string())); // Specify the end sequence used in the prompt
        payload.insert("remove_end_sequence".to_string(), Value::Bool(true)); // Omit end sequence from response
        payload.insert("remove_input".to_string(), Value::Bool(true)); // Omit input from response
        payload.insert("max_length".to_string(), Value::Number(Number::from(200))); // Max length of 200 chars

        // Configure repetition penalty (how likely the model is to repeat a word multiple times)
        payload.insert(
            "repetition_penalty".to_string(),
            Value::Number(
                Number::from_f64(repetition_penalty as f64)
                    .expect("Couldn't parse repetition penalty to numerical value."),
            ),
        );

        // Configure temperature (basically how different each answer should be, i.e. 0 it will be the same every time and 1 every post will be wildly different.)
        payload.insert(
            "temperature".to_string(),
            Value::Number(
                Number::from_f64(temperature as f64)
                    .expect("Couldn't parse temperature to numerical value."),
            ),
        );

        // Configure the reqwest client
        let client = reqwest::Client::new();

        // Configure the client and make the request to the GPT API
        let response = client
            .post(&self.api_url)
            .header(AUTHORIZATION, format!("Token {}", self.api_token))
            .header(CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await
            .expect("Couldn't query GPT API.");

        // Get the response text
        let response_text = response.text().await.unwrap();

        // Parse the response into JSON (generically but would probably be better practice
        // to declare a struct and deserialise the response into that struct)
        let response_json: Value = serde_json::from_str(&response_text)?;

        // Retrieve the actual generated post
        let response_content = response_json["generated_text"].to_string();

        Ok(response_content)
    }
}
