use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Parser)]
#[clap(author = "st3v3nhunt", version, about)]
/// CLI Sotck Info Retriever
struct Args {
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
    /// Type of request to make for symbol
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Price of symbol
    Price {
        /// Symbol to retrieve info about
        #[clap(forbid_empty_values = true)]
        symbol: String,
    },
    /// Company info
    Quote {
        /// Symbol to retrieve info about
        #[clap(forbid_empty_values = true)]
        symbol: String,
    },
}

#[derive(Deserialize, Debug)]
struct ShortQuote {
    symbol: String,
    price: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let args = Args::parse();
    println!("{:?}", args);
    match args.command {
        Commands::Price { symbol } => match get_price(&api_key, &symbol).await? {
            Some(r) => println!("Stock '{}' has price {}.", r.symbol, r.price),
            None => println!("No result returned for '{}'", symbol),
        },
        Commands::Quote { symbol } => {
            println!("Getting company quote for {}", symbol);
        }
    }

    Ok(())
}

async fn get_price(api_key: &str, symbol: &str) -> Result<Option<ShortQuote>> {
    println!("Getting price");
    let resp = reqwest::get(format!(
        "https://financialmodelingprep.com/api/v3/quote-short/{symbol}?apikey={api_key}",
        symbol = symbol,
        api_key = api_key
    ))
    .await?;
    match resp.status() {
        reqwest::StatusCode::OK => {
            let data = resp.json::<Vec<ShortQuote>>().await?;
            println!("Response data: {:?}", data);
            Ok(data.into_iter().nth(0))
        }
        _ => {
            println!(
                "Response for '{}' returned status code '{}' and was not parsable.",
                symbol,
                resp.status().as_str()
            );
            Ok(None)
        }
    }
}
