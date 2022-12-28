use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::helper;
use crate::rest::RestHandler;
use super::spec::Spec;

// Constant
const JOB_ENDPOINT: &str = "v1/jobs";

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
    pub async fn get_job_meta(&self, handler: &RestHandler) -> Result<(Option<Vec<String>>, Option<Vec<String>>), Error> {
        let spec = Spec::read(&self.name, handler).await?;

        Ok((spec.parameterized.meta_required, spec.parameterized.meta_optional))
    }

    pub async fn dispatch_job(&self, handler: &RestHandler, metas: HashMap<String, String>) -> Result<(), Error> {
        let endpoint = format!("v1/job/{}/dispatch", self.id);

        // transform the hashmap into a json in order to convert it to a base64
        let map_json = helper::to_json(&metas)?;
        let payload = helper::to_base64(map_json);

        let payload = DispatchPayload { payload, metas };

        // send the dispatch to nomad
        handler.post(&endpoint, payload).await?;

        Ok(())
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

    Ok(jobs)
}
