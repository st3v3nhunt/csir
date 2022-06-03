use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::env;

const URL: &str = "https://financialmodelingprep.com/api/v3";

#[derive(Debug, Parser)]
#[clap(author = "st3v3nhunt", version, about)]
/// CLI Sotck Info Retriever
pub struct Args {
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
    /// Type of request to make for symbol
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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

struct ReqInfo {
    segment: String,
    symbol: String,
}

pub async fn make_request(commands: &Commands) -> Result<()> {
    let req_info = match commands {
        Commands::Price { symbol } => ReqInfo {
            segment: "quote-short".to_string(),
            symbol: symbol.to_string(),
        },
        Commands::Quote { symbol } => ReqInfo {
            segment: "quote".to_string(),
            symbol: symbol.to_string(),
        },
    };
    println!("Getting price for {}", req_info.symbol);
    let api_key = env::var("API_KEY")?;
    let resp = reqwest::get(format!(
        "{URL}/{segment}/{symbol}?apikey={api_key}",
        segment = req_info.segment,
        symbol = req_info.symbol
    ))
    .await?;
    match resp.status() {
        reqwest::StatusCode::OK => {
            let data = resp.json::<Vec<ShortQuote>>().await?;
            println!("Response data: {:?}", data);
            let item = data.into_iter().nth(0).unwrap();
            println!("Stock '{}' has price {}.", item.symbol, item.price);
        }
        _ => {
            println!(
                "Response for '{}' returned status code '{}'.",
                req_info.symbol,
                resp.status().as_str()
            );
            println!("{:#?}", resp);
        }
    }
    Ok(())
}
