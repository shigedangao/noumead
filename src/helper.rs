use serde::Serialize;
use base64;
use crate::error::Error;

pub fn to_json<T: Serialize>(arg: T) -> Result<String, Error> {
    let res = serde_json::to_string(&arg)
        .map_err(|err| Error::Serialize(err.to_string()))?;

    Ok(res)
}

pub fn to_base64(arg: String) -> String {
    base64::encode(arg)
}
