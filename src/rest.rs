use std::env;
use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::error::Error;

// Constant
const NOMAD_ADDR_ENV: &str = "NOMAD_ADDR";

#[derive(Debug)]
pub struct RestHandler {
    base_url: String,
    token: Option<String>
}

impl RestHandler {
    /// Create a new connection options
    ///
    /// # Arguments
    ///
    /// * `base_url` - String
    /// * `token` - Option<String>
    pub fn new(base_url: Option<String>, token: Option<String>) -> Result<RestHandler, Error> {
        let url = match base_url {
            Some(u) => u,
            None => env::var(NOMAD_ADDR_ENV)?
        };

        Ok(RestHandler {
            base_url: url,
            token
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, Error> {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, endpoint);

        println!("{:?}", url);

        let mut req = client.get(url);

        if let Some(token) = self.token.as_ref() {
            req = req.header("X-Nomad-Token", token);
        }

        let res = req.send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }
}

