mod cli;
mod requests;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Args, Commands};
use dotenv::dotenv;
use std::env;

fn check_env() -> Result<()> {
    dotenv().ok();
    env::var("API_KEY").with_context(|| "env var API_KEY must be set")?;
    env::var("REMAINING_CALLS_API_URL")
        .with_context(|| "env var REMAINING_CALLS_API_URL must be set")?;
    Ok(())
}

pub async fn run() -> Result<()> {
    check_env().with_context(|| "environment was not set up correctly")?;

    let args = Args::parse();
    // TODO: only print when debug level set
    println!("{:?}", args);
    match &args.command {
        Commands::Price { symbols } => requests::get_price(&symbols).await?,
        Commands::Quote { symbols } => requests::get_quote(&symbols).await?,
        Commands::PriceChange { period, symbols } => {
            requests::get_price_change(symbols, period).await?
        }
        Commands::RemainingCalls => requests::get_remaining_calls().await?,
    }

    Ok(())
}
