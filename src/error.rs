use std::env::VarError;

use inquire::InquireError;

#[derive(Debug)]
pub enum Error {
    MissingEnv(String),
    NomadReqErr(String),
    Serialize(String),
    ScenarioFinished
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingEnv(msg) => write!(f, "Unable to find environment variable due to: {msg}"),
            Error::NomadReqErr(msg) => write!(f, "Nomad returns an error: {msg}"),
            Error::Serialize(msg) => write!(f, "Error while serializing data: {msg}"),
            Error::ScenarioFinished => write!(f, "No option selected. Terminating the program")
        }
    }
}

impl std::error::Error for Error {}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Error::MissingEnv(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NomadReqErr(err.to_string())
    }
}

impl From<InquireError> for Error {
    fn from(_: InquireError) -> Self {
        Error::ScenarioFinished
    }
}
