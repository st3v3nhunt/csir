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

fn check_env() {
    dotenv().ok();
    env::var("API_KEY").expect("API_KEY must be set");
}

#[tokio::main]
async fn main() -> Result<()> {
    check_env();

    let args = Args::parse();
    println!("{:?}", args);
    match args.command {
        Commands::Price { symbol } => match get_price(&symbol).await? {
            Some(r) => println!("Stock '{}' has price {}.", r.symbol, r.price),
            None => println!("No result returned for '{}'", symbol),
        },
        Commands::Quote { symbol } => get_quote(&symbol).await?,
    }

    Ok(())
}

const URL: &str = "https://financialmodelingprep.com/api/v3";

async fn get_quote(symbol: &str) -> Result<()> {
    println!("Getting company quote for {}", symbol);
    let api_key = env::var("API_KEY")?;
    let resp = reqwest::get(format!(
        "{URL}/quote/{symbol}?apikey={api_key}",
        api_key = api_key,
        symbol = symbol
    ))
    .await?;
    println!("{:#?}", resp);
    match resp.status() {
        reqwest::StatusCode::OK => {
            let data = resp.json::<Vec<ShortQuote>>().await?;
            println!("Response data: {:?}", data);
            let item = data.into_iter().nth(0).unwrap();
            println!("{}", item.symbol);
        }
        _ => {
            println!(
                "Response for '{}' returned status code '{}' and was not parsable.",
                symbol,
                resp.status().as_str()
            );
        }
    }
    Ok(())
}

async fn get_price(symbol: &str) -> Result<Option<ShortQuote>> {
    println!("Getting price");
    let api_key = env::var("API_KEY")?;
    let resp = reqwest::get(format!(
        "{URL}/quote-short/{symbol}?apikey={api_key}",
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
