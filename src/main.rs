mod cli;
mod nomad;
mod error;
mod rest;
mod inquiry;
mod helper;

use crossterm::style::Stylize;
use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = Cli::new()
        .init_rest_handler()?
        .run().await {
            println!("{} {}", "An error occurred:".red(), err.to_string().bold())
    }

    Ok(())
}
