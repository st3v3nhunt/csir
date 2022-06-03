use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::env;

use request::{Args, Commands};
mod request;

fn check_env() {
    dotenv().ok();
    env::var("API_KEY").expect("API_KEY must be set");
}

#[tokio::main]
async fn main() -> Result<()> {
    check_env();

    let args = Args::parse();
    println!("{:?}", args);
    match &args.command {
        Commands::Price { .. } => request::make_request(&args.command).await?,
        Commands::Quote { .. } => request::make_request(&args.command).await?,
    }

    Ok(())
}
