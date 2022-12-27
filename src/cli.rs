use std::collections::HashMap;

use clap::Parser;
use crate::error::Error;
use crate::rest::RestHandler;
use crate::nomad;
use crate::inquiry;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    follow: bool,

    #[arg(short, long)]
    nomad_url: Option<String>,

    #[arg(short, long)]
    token: Option<String>
}

#[derive(Debug)]
pub struct Cli {
    args: Args
}

impl Cli {
    /// Create a new cli by parsing the arguments
    pub fn new() -> Cli {
        let args = Args::parse();

        Cli { args }
    }

    /// Run the CLI with the provided arguments
    ///
    /// # Arguments
    /// * `&self` - Cli
    pub async fn run(&self) -> Result<(), Error> {
        let rest_handler = RestHandler::new(
            self.args.nomad_url.to_owned(),
            self.args.token.to_owned()
        )?;

        let jobs = nomad::job::get_nomad_job_list(&rest_handler).await?;
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
        let (required, optionals) = job.get_job_meta(&rest_handler).await?;

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
        job.dispatch_job(&rest_handler, required_value).await?;

        Ok(())
    }
}
