// src/okta/client.rs

use reqwest::Client as HttpClient;
use std::env;

pub struct Client {
    http_client: HttpClient,
    base_url: String,
    api_token: String,
}

impl Client {
    pub fn new_from_env() -> Self {
        let base_url = env::var("OKTA_BASE_URL").expect("OKTA_BASE_URL must be set");
        let api_token = env::var("OKTA_API_TOKEN").expect("OKTA_API_TOKEN must be set");

        Client {
            http_client: HttpClient::new(),
            base_url,
            api_token,
        }
    }

    // Add methods for authentication and API requests here
}