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
        Commands::PriceChange { symbols } => requests::get_price_change(&symbols).await?,
    }

    Ok(())
}
