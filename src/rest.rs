use reqwest::{Client, RequestBuilder};
use tokio::time::{sleep, Duration};
use serde::{de::DeserializeOwned, Serialize};
use crate::error::{Error, self};

// Constant
const RETRY_LINEAR_SLEEP: u64 = 1000;
const MAX_RETRY: usize = 8;

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
        let Some(url) = base_url else {
            return Err(Error::MissingEnv(error::MISSING_BASE_URL_ERR.to_string()))
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
    /// * `endpoint` - S
    pub async fn get<T, S>(&self, endpoint: S) -> Result<T, Error>
        where
            T: DeserializeOwned,
            S: AsRef<str> + std::fmt::Display
    {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut req = client.get(url);
        if let Some(token) = self.token.as_ref() {
            req = req.header("X-Nomad-Token", token);
        }

        let res = retry(req, MAX_RETRY).await?;

        Ok(res)
    }

    /// Send a delete request to the targeted endpoint
    ///
    /// # Arguments
    ///
    /// * `&self` - RestHandler
    /// * `endpoint` - S
    pub async fn delete<S>(&self, endpoint: S) -> Result<(), Error>
        where
            S: AsRef<str> + std::fmt::Display
    {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut req = client.delete(url);
        if let Some(token) = self.token.as_ref() {
            req = req.header("X-Nomad-Token", token);
        }

        req.send().await?;

        Ok(())
    }

    /// Prepare and send a post request to the nomad api
    ///
    /// # Arguments
    ///
    /// * `&self` - RestHandler
    /// * `endpoint` - S
    /// * `payload` - T
    pub async fn post<T, O, S>(&self, endpoint: S, payload: T) -> Result<O, Error>
        where
            T: Serialize,
            O: DeserializeOwned,
            S: AsRef<str> + std::fmt::Display
    {
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

        let output = res.json::<O>().await?;

        Ok(output)
    }
}

/// Retry an http request. Due to the fact that the nomad endpoint might returns nothing
/// or a null json value as something might not be available yet, we need to retry some request for some time.
/// So far, the implementation is based on a linear retry. Should it be not enough it'd be better to implement
/// an exponential backoff
///
/// # Arguments
///
/// * `req` - ReqBuilder
/// * `max_retry` - usize
async fn retry<T: DeserializeOwned>(req: RequestBuilder, max_retry: usize) -> Result<T, Error> {
    for idx in 1..max_retry {
        let Some(req) = req.try_clone() else {
            return Err(Error::NomadReqErr(error::REQ_BUILD_FAIL_ERR.to_string()));
        };

        let res = match req.send().await {
            Ok(r) => r,
            Err(err) => {
                if idx < MAX_RETRY - 1 {
                    sleep(Duration::from_millis(RETRY_LINEAR_SLEEP)).await;
                    continue
                }

                return Err(Error::from(err));
            }
        };

        // try to parse the output data
        match res.json::<T>().await {
            Ok(res) => return Ok(res),
            Err(err) => {
                if idx < MAX_RETRY - 1 {
                    sleep(Duration::from_millis(RETRY_LINEAR_SLEEP)).await;
                    continue
                }

                return Err(Error::from(err));
            }
        }
    }

    Err(Error::MaxRetry)
}
