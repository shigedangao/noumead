use serde::Deserialize;
use crate::{error::Error, rest::RestHandler};

const SPEC_ENDPOINT: &str = "v1/job";

#[derive(Debug, Deserialize)]
pub(crate) struct Spec {
    #[serde(rename(deserialize = "ParameterizedJob"))]
    pub parameterized: Parameterized
}

#[derive(Debug, Deserialize)]
pub(crate) struct Parameterized {
    #[serde(rename(deserialize = "MetaRequired"))]
    pub meta_required: Option<Vec<String>>,

    #[serde(rename(deserialize = "MetaOptional"))]
    pub meta_optional: Option<Vec<String>>
}

impl Spec {
    /// Read the spec of the selected job
    ///
    /// # Arguments
    ///
    /// * `name` - &str
    /// * `base_url` - &str
    /// * `handler` - &RestHandler
    pub async fn read(name: &str, handler: &RestHandler) -> Result<Spec, Error> {
        let endpoint = format!("{}/{}", SPEC_ENDPOINT, name);
        let spec = handler.get::<Spec>(&endpoint).await?;

        Ok(spec)
    }
}
