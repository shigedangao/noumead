use std::env::VarError;

#[derive(Debug)]
pub enum Error {
    MissingEnv(String),
    NomadReqErr(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingEnv(msg) => write!(f, "Unable to find environment variable due to: {msg}"),
            Error::NomadReqErr(msg) => write!(f, "Nomad returns an error: {msg}")
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