use serde::Deserialize;
use crate::error::Error;
use crate::helper::Base64;
use crate::log::Logger;
use crate::rest::RestHandler;

pub enum StdKind {
    Stdout,
    Stderr
}

#[derive(Debug, Deserialize)]
pub(crate) struct StreamLog {
    #[serde(rename(deserialize = "Offset"))]
    offset: Option<i64>,

    #[serde(rename(deserialize = "Data"))]
    data: Option<String>
}

/// Fetch the job log by using the nomad fs/logs endpoint.
/// Note that in Nomad the data is stored as a base64 value which needed to be decoded.
/// Should the log endpoint returns nothing this means that Nomad has nothing to returns...
///
/// For long log we need to set the offset.
///
/// # Arguments
///
/// * `req` - &RestHandler
/// * `id` - &str
/// * `task_name` - &str
/// * `std_kind` - StdKind
/// * `offset` - i64
pub async fn stream_dispatch_job_log(
    req: &RestHandler,
    id: &str,
    task_name: &str,
    std_kind: StdKind,
    offset: i64,
    prev_offset: &mut i64
) -> Result<i64, Error> {
    let std_kind_str = match std_kind {
        StdKind::Stdout => "stdout",
        StdKind::Stderr => "stderr"
    };

    let endpoint = format!("v1/client/fs/logs/{id}?task={task_name}&type={std_kind_str}&offset={offset}");
    // /!\ If nomad returns nothing this could cause reqwest to thrown an error as it could not
    //     deserialize the result. As a result we skip the error altogether.
    let Ok(res) = req.get::<StreamLog, _>(&endpoint).await else {
        return Ok(0);
    };

    let res_offset = res.offset.unwrap_or(0);
    if let Some(data) = res.data {
        let content = String::from_base64(data)
            .map_err(|err| Error::ScenarioErr(err.to_string()))?;

        if prev_offset != &res_offset {
            match std_kind {
                StdKind::Stdout => Logger::notice(content),
                StdKind::Stderr => Logger::error("", content)
            }

            *prev_offset = res_offset;
        }
    }

    Ok(res_offset)
}
