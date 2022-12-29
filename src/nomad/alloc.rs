use std::collections::HashMap;
use serde::Deserialize;
use futures::future;
use tokio::time::{Duration, sleep};
use crate::{error::Error, rest::RestHandler};
use super::stream;

// Constant
const SLEEP: u64 = 100;
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
    /// Fetch the information about the allocations
    ///
    /// # Arguments
    ///
    /// * `job_id` - &str
    /// * `rest_handler` - &RestHandler
    pub async fn fetch(job_id: &str, rest_handler: &RestHandler) -> Result<Allocation, Error> {
        let endpoint = format!("v1/job/{}/allocations", job_id);
        let mut allocs: Vec<Allocation> = rest_handler.get(&endpoint).await?;

        // for dispatched job there's one allocation per dispatch...
        let alloc = allocs.pop();
        if let Some(al) = alloc {
            return Ok(al);
        }

        Err(Error::ScenarioErr(MISSING_ALLOCATION.to_string()))
    }

    /// Get the allocation logs by calling the nomad endpoint repetitively until the allocation finish to run
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    /// * `task_name` - &str
    /// * `rest_handler` - &RestHandler
    pub async fn get_allocation_logs(self, task_name: &str, rest_handler: &RestHandler) -> Result<(), Error> {
        let mut stdout_offset = 0;
        let mut stderr_offset = 0;

        loop {
            // fetch the logs
            let offsets = future::join_all(vec![
                stream::stream_dispatch_job_log(rest_handler, &self.alloc_id, task_name, stream::StdKind::Stdout, stdout_offset),
                stream::stream_dispatch_job_log(rest_handler, &self.alloc_id, task_name, stream::StdKind::Stderr, stderr_offset)
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
            let alloc = Allocation::fetch(&self.job_id, &rest_handler).await?;

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
    pub fn get_tasks_name(&self) -> Vec<&String> {
        let v = self.task_states.keys();
        let tasks = Vec::from_iter(v);

        tasks
    }
}
