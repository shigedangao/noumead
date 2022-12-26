mod cli;
mod nomad;
mod error;
mod rest;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::new().run().await?;

    Ok(())
}
