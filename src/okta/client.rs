use okta::Client;
use std::env;

pub struct OktaClient {
    client: Client,
}

impl OktaClient {
    pub fn new() -> Self {
        let api_key = env::var("OKTA_API_TOKEN").expect("OKTA_API_TOKEN must be set");
        let url = env::var("OKTA_ORG_URL").expect("OKTA_ORG_URL must be set");

        let client = Client::new(api_key).with_host_override(url).to_owned();

        Self { client }
    }

    // Add methods to interact with Okta API
    pub fn get_client(&self) -> &Client {
        &self.client
    }
}
