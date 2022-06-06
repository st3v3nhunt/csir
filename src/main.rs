mod cli;
mod requests;

use anyhow::Result;
use clap::Parser;
use cli::{Args, Commands};
use dotenv::dotenv;
use std::env;

fn check_env() {
    dotenv().ok();
    env::var("API_KEY").expect("API_KEY must be set");
    env::var("REMAINING_CALLS_API_URL").expect("REMAINING_CALLS_API_URL must be set");
}

#[tokio::main]
async fn main() -> Result<()> {
    check_env();

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
