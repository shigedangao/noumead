use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::helper;
use crate::rest::RestHandler;
use crate::inquiry::ItemName;
use super::spec::Spec;
use super::dispatch::DispatchRes;

// Constant
const JOB_ENDPOINT: &str = "v1/jobs";
const JOBS_NOT_FOUND_ERR: &str = "No jobs with parameterized options has been founded";

#[derive(Serialize)]
struct DispatchPayload {
    #[serde(rename(serialize = "Payload"))]
    payload: String,
    #[serde(rename(serialize = "Meta"))]
    metas: HashMap<String, String>
}

#[derive(Debug, Deserialize)]
pub struct Job {
    #[serde(rename(deserialize = "ID"))]
    id: String,

    #[serde(rename(deserialize = "Name"))]
    name: String,

    #[serde(rename(deserialize = "ParameterizedJob"))]
    parameterized: bool
}

impl Job {
    /// Get the job meta from a selected job
    ///
    /// # Arguments
    ///
    /// * `&self` - &Job
    /// * `handler` - &RestHandler
    pub async fn get_job_meta(&self, handler: &RestHandler) -> Result<(Option<Vec<String>>, Option<Vec<String>>), Error> {
        let spec = Spec::get(&self.name, handler).await?;

        Ok((spec.parameterized.meta_required, spec.parameterized.meta_optional))
    }

    /// Dispatch a job to nomad with the selected metas
    ///
    /// # Arguments
    ///
    /// * `&self` - &Job
    /// * `handler` - &RestHandler
    /// * `metas` - HashMap<String, String>
    pub async fn dispatch_job(&self, handler: &RestHandler, metas: HashMap<String, String>) -> Result<(), Error> {
        let endpoint = format!("v1/job/{}/dispatch", self.id);

        // transform the hashmap into a json in order to convert it to a base64
        let map_json = helper::to_json(&metas)?;
        let payload = helper::to_base64(map_json);

        let payload = DispatchPayload { payload, metas };

        // send the dispatch to nomad
        let res: DispatchRes = handler.post(&endpoint, payload).await?;
        println!("Job dispatch with name {}", res.dispatch_id);

        Ok(())
    }
}

impl ItemName for Job {
    fn get_name(&self) -> &str {
        &self.name
    }
}

/// Get a list of nomad job
///
/// # Arguments
///
/// * `handler` - &RestHandler
pub async fn get_nomad_job_list(handler: &RestHandler) -> Result<Vec<Job>, Error> {
    let endpoint = format!("{}?meta=true&namespace=*", JOB_ENDPOINT);
    let jobs: Vec<Job> = handler.get::<Vec<Job>>(&endpoint)
        .await?
        .into_iter()
        .filter(|j| j.parameterized)
        .collect();

    if jobs.is_empty() {
        return Err(Error::ScenarioErr(JOBS_NOT_FOUND_ERR.to_string()));
    }

    Ok(jobs)
}
