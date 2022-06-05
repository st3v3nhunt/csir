use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::env;

use crate::requests::get_price;
use request::{Args, Commands};
mod request;
mod requests;

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
        Commands::Price { symbols } => get_price(&symbols).await?,
        Commands::Quote { symbols } => request::get_quote(&symbols).await?,
        Commands::PriceChange { symbols } => request::get_price_change(&symbols).await?,
    }

    Ok(())
}
