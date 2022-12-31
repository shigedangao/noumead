use std::collections::HashMap;
use clap::Args;
use crossterm::style::Stylize;
use async_trait::async_trait;
use crate::{nomad, inquiry, error::Error};
use super::Run;

// constant
const SELECTED_JOB_NOT_FOUND: &str = "Unable to found the selected job";

#[derive(Args, Debug)]
pub struct DispatchArgs {
    #[arg(short, long)]
    follow: bool,
}

#[async_trait]
impl Run for DispatchArgs {
    async fn run(&self, cli: &super::Cli) -> Result<(), crate::error::Error> {
        let jobs = nomad::job::get_nomad_job_list(&cli.rest_handler).await?;
        let (_, idx) = inquiry::select(&jobs, "Select the job that you want to dispatch")?;

        let Some(job) = jobs.get(idx) else {
            return Err(Error::ScenarioErr(SELECTED_JOB_NOT_FOUND.to_string()));
        };

        let (required, optionals) = job.get_job_meta(&cli.rest_handler).await?;

        let mut required_value = match required {
            Some(metas) => inquiry::prompt_vector(metas, "Input the required value for", true)?,
            None => HashMap::new()
        };

        let optional_value = match optionals {
            Some(metas) => inquiry::prompt_vector(metas, "Input value for optional", false)?,
            None => HashMap::new()
        };

        // merge the required_value with the optional_value
        required_value.extend(optional_value.into_iter());

        // dispatch the job
        let dispatch_res = job.dispatch_job(&cli.rest_handler, required_value).await?;
        // follow the log of the job dispatch
        if self.follow {
            let allocs = nomad::alloc::Allocation::fetch(&dispatch_res.dispatch_id, &cli.rest_handler).await?;
            let tasks_name = allocs.get_tasks_name();

            // ask for the list of task to choose
            let (selected_task, _) = inquiry::select(&tasks_name, "Select the task to log")?;
            // get the logs for the targeted allocations
            allocs.get_allocation_logs(&selected_task, &cli.rest_handler).await?;

            println!("{}", "Dispatching done".green());

            return Ok(());
        }

        println!("{}", "Dispatching done".green());

        Ok(())
    }
}
