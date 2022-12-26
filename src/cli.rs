use clap::Parser;
use crate::error::Error;
use crate::rest::RestHandler;
use crate::nomad;

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

        nomad::job::get_nomad_job_list(&rest_handler).await?;

        Ok(())
    }
}
