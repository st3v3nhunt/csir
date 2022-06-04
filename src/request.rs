use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_json::Value;
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
    /// Real-time stock price
    Price {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true)]
        symbols: Vec<String>,
    },
    /// Percentage price change for multiple symbols
    PriceChange {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true)]
        symbols: Vec<String>,
    },
    /// Companies quote
    Quote {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true)]
        symbols: Vec<String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct ShortQuote {
    symbol: String,
    price: f64,
}

pub async fn get_price(symbols: &Vec<String>) -> Result<()> {
    let api_key = env::var("API_KEY")?;
    let resp = reqwest::get(format!(
        "{URL}/{segment}/{symbols}?apikey={api_key}",
        segment = "quote-short",
        symbols = symbols.join(",")
    ))
    .await?;
    let data = resp.json::<Vec<ShortQuote>>().await?;
    println!("Response data: {:?}", data);
    let item = data.into_iter().nth(0).unwrap();
    println!("Stock '{}' has price {}.", item.symbol, item.price);
    Ok(())
}

pub async fn get_price_change(symbols: &Vec<String>) -> Result<()> {
    let api_key = env::var("API_KEY")?;
    let resp = reqwest::get(format!(
        "{URL}/{segment}/{symbols}?apikey={api_key}",
        segment = "stock-price-change",
        symbols = symbols.join(",")
    ))
    .await?;
    let data = resp.json::<Vec<Value>>().await?;
    println!("Response data: {:?}", data);
    let item = data.into_iter().nth(0).unwrap();
    println!(
        "Stock '{}' has changed price by {}% over the course of 1 day.",
        item["symbol"], item["1D"]
    );
    Ok(())
}

pub async fn get_quote(symbols: &Vec<String>) -> Result<()> {
    let api_key = env::var("API_KEY")?;
    let resp = reqwest::get(format!(
        "{URL}/{segment}/{symbols}?apikey={api_key}",
        segment = "quote",
        symbols = symbols.join(",")
    ))
    .await?;
    let data = resp.json::<Vec<ShortQuote>>().await?;
    println!("Response data: {:?}", data);
    let item = data.into_iter().nth(0).unwrap();
    println!("Stock '{}' has price {}.", item.symbol, item.price);
    Ok(())
}
