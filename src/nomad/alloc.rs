use std::collections::HashMap;
use serde::Deserialize;
use futures::future;
use tokio::time::{Duration, sleep};
use crate::{error::Error, rest::RestHandler};
use crate::log::Logger;
use super::stream;

// Constant
const SLEEP: u64 = 100;
const ALLOCATION_MAX_RETRY: usize = 5;
const MISSING_ALLOCATION: &str = "Unable to found an allocation for the given dispatch";

#[derive(Debug, Deserialize)]
pub(crate) struct Allocation {
    #[serde(rename(deserialize = "TaskStates"))]
    task_states: HashMap<String, TaskState>,

    #[serde(rename(deserialize = "ID"))]
    alloc_id: String,

    #[serde(rename(deserialize = "JobID"))]
    job_id: String
}

#[derive(Debug, Deserialize)]
pub struct TaskState {
    #[serde(rename(deserialize = "FinishedAt"))]
    finished_at: Option<String>
}

impl Allocation {
    /// Try to fetch the allocations with a maximum number of retry
    ///
    /// # Arguments
    ///
    /// * `job_id` - &str
    /// * `rest_handler` - &rest_handler
    pub async fn fetch(job_id: &str, rest_handler: &RestHandler) -> Result<Vec<Allocation>, Error> {
        for idx in 1..ALLOCATION_MAX_RETRY {
            let endpoint = format!("v1/job/{}/allocations", job_id);
            let allocs: Vec<Allocation> = rest_handler.get(&endpoint).await?;

            if !allocs.is_empty() {
                return Ok(allocs);
            }

            // otherwise pause for a bit
            Logger::warn(format!("Retry to get the allocation for the {idx}"));
            sleep(Duration::from_millis(SLEEP)).await;
        }

        Err(Error::MaxRetry)
    }

    /// Fetch a single allocation
    ///
    /// # Arguments
    ///
    /// * `job_id` - &str
    /// * `rest_handler` - &RestHandler
    pub async fn fetch_single_alloc(job_id: &str, rest_handler: &RestHandler) -> Result<Allocation, Error> {
        let mut allocs = Allocation::fetch(job_id, rest_handler).await?;
        let Some(alloc) = allocs.pop() else {
            return Err(Error::ScenarioErr(MISSING_ALLOCATION.to_string()));
        };

        Ok(alloc)
    }

    /// Get the allocation logs by calling the nomad endpoint repetitively until the allocation finish to run
    ///
    /// # Arguments
    ///
    /// * `self` - Allocation
    /// * `task_name` - &str
    /// * `rest_handler` - &RestHandler
    pub async fn get_allocation_logs(self, task_name: &str, rest_handler: &RestHandler) -> Result<(), Error> {
        let mut stdout_offset = 0;
        let mut stderr_offset = 0;

        let mut prev_stdout_offset = -1;
        let mut prev_stderr_offset = -1;

        loop {
            // fetch the logs
            let offsets = future::join_all(vec![
                stream::stream_dispatch_job_log(rest_handler, &self.alloc_id, task_name, stream::StdKind::Stdout, stdout_offset, &mut prev_stdout_offset),
                stream::stream_dispatch_job_log(rest_handler, &self.alloc_id, task_name, stream::StdKind::Stderr, stderr_offset, &mut prev_stderr_offset)
            ]).await;

            for (idx, items) in offsets.into_iter().enumerate() {
                match items {
                    Ok(offset) => {
                        if idx == 0 {
                            stdout_offset = offset
                        } else {
                            stderr_offset = offset
                        }
                    },
                    Err(err) => return Err(err)
                }
            }

            // call the fetch endpoint again to get the update status of the allocation
            let alloc = Allocation::fetch_single_alloc(&self.job_id, rest_handler).await?;
            // call the allocation endpoint to check whether the task has finish
            let Some(task) = alloc.task_states.get(task_name) else {
                return Err(Error::MissingTask);
            };
            // Task has finish no need to get the log anymore
            if task.finished_at.is_some() {
                return Ok(())
            }

            // Otherwise pause for some times.
            sleep(Duration::from_millis(SLEEP)).await;
        }
    }

    /// A nomad job can contains multiple task (aka container in Kubernetes world)
    /// as such if we want to log we need to get the list of available task name.
    ///
    /// # Arguments
    ///
    /// * `&self` - Allocation
    pub fn get_tasks_name(&self) -> Vec<&String> {
        let v = self.task_states.keys();

        Vec::from_iter(v)
    }
}
