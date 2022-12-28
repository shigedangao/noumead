use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct DispatchRes {
    #[serde(rename(deserialize = "DispatchedJobID"))]
    pub dispatch_id: String
}
