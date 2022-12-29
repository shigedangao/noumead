use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct DispatchPayload {
    #[serde(rename(serialize = "Payload"))]
    pub payload: String,
    #[serde(rename(serialize = "Meta"))]
    pub metas: HashMap<String, String>
}


#[derive(Debug, Deserialize)]
pub struct DispatchRes {
    #[serde(rename(deserialize = "EvalID"))]
    pub eval_id: String,

    #[serde(rename(deserialize = "DispatchedJobID"))]
    pub dispatch_id: String
}
