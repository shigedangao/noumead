use std::env::VarError;
use inquire::InquireError;

// Error constant for scenario error
pub const SELECTED_JOB_NOT_FOUND_ERR: &str = "Unable to found the selected job";
pub const NO_RUNNING_JOB_ERR: &str = "No running job has been found";
pub const MISSING_ALLOCATION_ERR: &str = "Unable to found an allocation for the given dispatch";
pub const JOBS_NOT_FOUND_ERR: &str = "No jobs with parameterized options has been founded";
pub const SELECTED_ITEM_NOT_FOUND_ERR: &str = "Unable to found the selected item";
pub const MISSING_REQUIRED_FIELD_ERR: &str = "You must fill this field as the value is required";
pub const REQ_BUILD_FAIL_ERR: &str = "Failed to build request";
pub const MISSING_BASE_URL_ERR: &str = "Failed to get the url of the nomad server";

#[derive(Debug)]
pub enum Error {
    MissingEnv(String),
    NomadReqErr(String),
    Serialize(String),
    Dispatch,
    ScenarioFinished,
    ScenarioErr(String),
    MissingTask,
    MaxRetry
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingEnv(msg) => write!(f, "Unable to find environment variable due to: {msg}"),
            Error::NomadReqErr(msg) => write!(f, "An error occurred while querying the HTTP endpoint of Nomad: {msg}"),
            Error::Serialize(msg) => write!(f, "Error while serializing data: {msg}"),
            Error::Dispatch => write!(f, "Job dispatching has fail"),
            Error::ScenarioFinished => write!(f, "No option selected. Terminating the program"),
            Error::ScenarioErr(msg) => write!(f, "The command has stopped due to: {msg}"),
            Error::MissingTask => write!(f, "The selected task could not be found"),
            Error::MaxRetry => write!(f, "Max retry has been achieved when fetching data")
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
