use std::env;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use crate::error::Error;

// Constant
const NOMAD_ADDR_ENV: &str = "NOMAD_ADDR";

#[derive(Debug, Default)]
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

    /// Prepare and send a get request to the nomad api
    ///
    /// # Arguments
    ///
    /// * `&self` - RestHandler
    /// * `endpoint` - &str
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, Error> {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, endpoint);

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

    /// Prepare and send a post request to the nomad api
    ///
    /// # Arguments
    ///
    /// * `&self` - RestHandler
    /// * `endpoint` - &str
    /// * `payload` - T
    pub async fn post<T: Serialize>(&self, endpoint: &str, payload: T) -> Result<(), Error> {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut req = client.post(url);
        if let Some(token) = self.token.as_ref() {
            req = req.header("X-Nomad-Token", token);
        }

        let res = req
            .json(&payload)
            .send()
            .await?;

        if res.status() != 200 {
            return Err(Error::Dispatch);
        }

        Ok(())
    }
}

