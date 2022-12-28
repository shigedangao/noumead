use std::collections::HashMap;
use clap::Args;
use async_trait::async_trait;
use crate::{nomad, inquiry, error::Error};
use super::Run;


#[derive(Args, Debug)]
pub struct DispatchArgs {
    #[arg(short, long)]
    follow: bool,
}

#[async_trait]
impl Run for DispatchArgs {
    async fn run(&self, cli: &super::Cli) -> Result<(), crate::error::Error> {
        let jobs = nomad::job::get_nomad_job_list(&cli.rest_handler).await?;
        let jobs_name: Vec<&str> = jobs.keys()
            .map(|k| k.as_str())
            .collect();

        let selected_job_name = inquiry::select(jobs_name, "Select the job that you want to dispatch")?;

        // get the selected job
        let selected_job = jobs.get(&selected_job_name);
        // @TODO change error
        if selected_job.is_none() {
            return Err(Error::ScenarioFinished);
        }

        let job = selected_job.unwrap();
        let (required, optionals) = job.get_job_meta(&cli.rest_handler).await?;

        let mut required_value = match required {
            Some(metas) => inquiry::prompt_vector(metas, "Input the required value for")?,
            None => HashMap::new()
        };

        let optional_value = match optionals {
            Some(metas) => inquiry::prompt_vector(metas, "Input value for optional")?,
            None => HashMap::new()
        };

        // merge the required_value with the optional_value
        required_value.extend(optional_value.into_iter());

        // dispatch the job
        job.dispatch_job(&cli.rest_handler, required_value).await?;

        Ok(())
    }
}
