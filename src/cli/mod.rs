use std::env;
use clap::{Parser, Subcommand};
use async_trait::async_trait;
use crate::error::Error;
use crate::rest::RestHandler;

mod dispatch;
mod stop;

// constant
const NOMAD_ADDR_ENV: &str = "NOMAD_ADDR";
const NOMAD_TOKEN_ENV: &str = "NOMAD_TOKEN";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    nomad_url: Option<String>,

    #[arg(short, long)]
    token: Option<String>,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Dispatch(dispatch::DispatchArgs),
    Stop(stop::StopArgs)
}

#[derive(Debug)]
pub struct Cli {
    args: Args,
    rest_handler: RestHandler
}

#[async_trait]
trait Run {
    /// Run the targeted command with the associated Cli instance
    ///
    /// # Arguments
    ///
    /// * `&self` - Self
    /// * `cli` - &Cli
    async fn run(&self, cli: &Cli) -> Result<(), Error>;
}

impl Cli {
    /// Create a new cli by parsing the arguments
    pub fn new() -> Cli {
        let mut args = Args::parse();
        args.fill_optional_values();

        Cli {
            args,
            rest_handler: RestHandler::default()
        }
    }

    /// Initialize the rest handler which is gonna be used to send request to Nomad
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Cli
    pub fn init_rest_handler(&mut self) -> Result<&Self, Error> {
        let rest_handler = RestHandler::new(
            self.args.nomad_url.to_owned(),
            self.args.token.to_owned()
        )?;

        self.rest_handler = rest_handler;

        Ok(self)
    }

    /// Run the CLI with the provided arguments
    ///
    /// # Arguments
    /// * `&self` - Cli
    pub async fn run(&self) -> Result<(), Error> {
        match &self.args.command {
            Commands::Dispatch(args) => args.run(self).await,
            Commands::Stop(args) => args.run(self).await
        }
    }
}

impl Args {
    /// Try to fill the optional value by looking at the environment variable
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Args
    fn fill_optional_values(&mut self) {
        if self.nomad_url.is_none() {
            if let Ok(url) = env::var(NOMAD_ADDR_ENV) {
                self.nomad_url = Some(url);
            }
        }

        if self.token.is_none() {
            if let Ok(token) = env::var(NOMAD_TOKEN_ENV) {
                self.token = Some(token);
            }
        }
    }
}
