use clap::Args;
use async_trait::async_trait;
use futures::future::join_all;
use crate::nomad::job::{self, Job};
use crate::{inquiry, log::Logger};
use crate::error::Error;
use super::Run;

// constant
const RUNNING_STATUS: &str = "running";
const NO_RUNNING_JOB_ERR: &str = "No running job has been founded";

#[derive(Args, Debug)]
pub struct StopArgs;

#[async_trait]
impl Run for StopArgs {
    async fn run(&self, cli: &super::Cli) ->  Result<(), crate::error::Error> {
        let jobs: Vec<Job> = job::get_nomad_job_list(&cli.rest_handler).await?
            .into_iter()
            .filter(|j| !j.parameterized && j.status == RUNNING_STATUS)
            .collect();

        if jobs.is_empty() {
            return Err(Error::ScenarioErr(NO_RUNNING_JOB_ERR.to_string()));
        }

        let (selected_jobs_name, _) = inquiry::multi_select(&jobs, "Select the jobs that you want to stop")?;

        let selected_jobs: Vec<Job> = jobs.into_iter()
            .filter(|j| selected_jobs_name.contains(&j.name))
            .collect();

        let mut tasks = Vec::new();
        for job in selected_jobs {
            tasks.push(cli.rest_handler.delete(format!("v1/job/{}", job.id)));
        }

        let res = join_all(tasks).await;
        for r in res {
            match r {
                Ok(_) => Logger::info("A job has been deleted"),
                Err(err) => Logger::error( "Unable to delete a job due to", err)
            }
        }

        Ok(())
    }
}
