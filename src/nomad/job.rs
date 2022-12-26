use serde::Deserialize;
use crate::error::Error;
use crate::rest::RestHandler;

// Constant
const JOB_ENDPOINT: &str = "/v1/jobs";

#[derive(Debug, Deserialize)]
pub struct Job {
    #[serde(rename(deserialize = "Name"))]
    name: String
}

/// Get a list of nomad job
///
/// # Arguments
///
/// * `handler` - &RestHandler
pub async fn get_nomad_job_list(handler: &RestHandler) -> Result<Vec<Job>, Error> {
    let endpoint = format!("{}?meta=true&namespace=*", JOB_ENDPOINT);
    let jobs = handler.get::<Vec<Job>>(&endpoint).await?;

    println!("{jobs:?}");

    Ok(vec![])
}
