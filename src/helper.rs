use serde::{Serialize};
use base64;
use crate::error::Error;

pub fn to_json<T: Serialize>(arg: T) -> Result<String, Error> {
    let res = serde_json::to_string(&arg)
        .map_err(|err| Error::Serialize(err.to_string()))?;

    Ok(res)
}

pub trait Base64 {
    /// Convert a string to a base64 string
    fn to_base64(&self) -> String;
    /// Convert a base64 string value to a string
    fn from_base64(b64: String) -> Result<String, Box<dyn std::error::Error>>;
}

impl Base64 for String {
    fn to_base64(&self) -> Self {
        base64::encode(&self)
    }

    fn from_base64(b64: String) -> Result<Self, Box<dyn std::error::Error>> {
        let res = base64::decode(b64)?;
        let st = String::from_utf8(res)?;

        Ok(st)
    }
}

