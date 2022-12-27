mod cli;
mod nomad;
mod error;
mod rest;
mod inquiry;
mod helper;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = Cli::new().run().await {
        panic!("{err}");
    }

    Ok(())
}
