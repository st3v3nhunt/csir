use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Parser)]
struct Cli {
    symbol: String,
}

#[derive(Deserialize, Debug)]
struct ShortQuote {
    symbol: String,
    price: f64,
    volume: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    println!("API_KEY: {}", api_key);
    let args = Cli::parse();
    println!("{:?}", args);
    let resp = reqwest::get(format!(
        "https://financialmodelingprep.com/api/v3/quote-short/{symbol}?apikey={api_key}",
        symbol = &args.symbol,
        api_key = api_key
    ))
    .await?;
    println!("{:?}", resp);
    let data = resp.json::<Vec<ShortQuote>>().await?;
    println!("{:#?}", data);
    let result = data.first().unwrap();
    println!(
        "Stock {} has price {}. Last volume {}",
        result.symbol, result.price, result.volume
    );
    Ok(())
}
