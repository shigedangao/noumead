use reqwest::{Client, RequestBuilder};
use tokio::time::{sleep, Duration};
use serde::{de::DeserializeOwned, Serialize};
use crate::error::Error;

// Constant
const RETRY_LINEAR_SLEEP: u64 = 1000;
const MAX_RETRY: usize = 8;
const REQ_BUILD_FAIL_ERR: &str = "Failed to build request";
const MISSING_BASE_URL_ERR: &str = "Failed to get the url of the nomad server";

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
            return Err(Error::MissingEnv(MISSING_BASE_URL_ERR.to_string()))
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

        let res = retry(req, MAX_RETRY).await?;

        Ok(res)
    }

    /// Prepare and send a post request to the nomad api
    ///
    /// # Arguments
    ///
    /// * `&self` - RestHandler
    /// * `endpoint` - &str
    /// * `payload` - T
    pub async fn post<T: Serialize, O: DeserializeOwned>(&self, endpoint: &str, payload: T) -> Result<O, Error> {
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
            return Err(Error::NomadReqErr(REQ_BUILD_FAIL_ERR.to_string()));
        };

        let req_res = req.send().await;
        if let Err(err) = req_res {
            if idx < MAX_RETRY - 1 {
                sleep(Duration::from_millis(RETRY_LINEAR_SLEEP)).await;
                continue
            }

            // Otherwise return an error
            return Err(Error::from(err));
        }

        // try to parse the output data
        let data = req_res.unwrap().json::<T>().await;
        if let Ok(d) = data {
            return Ok(d);
        }

        if idx < MAX_RETRY - 1 {
            sleep(Duration::from_millis(RETRY_LINEAR_SLEEP)).await;
            continue
        }

        // Otherwise return an error
        return Err(Error::MaxRetry);
    }

    return Err(Error::MaxRetry)
}
