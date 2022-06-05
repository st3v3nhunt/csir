use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use std::env;

const URL: &str = "https://financialmodelingprep.com/api/v3";

#[derive(Debug, Parser)]
#[clap(about = "CLI Stock Info Retriever")]
#[clap(author = "st3v3nhunt", version)]
pub struct Args {
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
    /// Type of request to make for symbol
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Real-time price of symbol(s)
    Price {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true, default_value = "", max_values = 1)]
        symbols: Vec<String>,
    },
    /// Percentage price change of symbol(s)
    PriceChange {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true, default_value = "")]
        symbols: Vec<String>,
    },
    /// Full quote for symbol(s)
    Quote {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true, default_value = "", max_values = 1)]
        symbols: Vec<String>,
    },
}

async fn make_request(segment: &str, symbols: &Vec<String>) -> Result<Response> {
    let api_key = env::var("API_KEY")?;
    let url = format!(
        "{URL}/{segment}/{symbols}?apikey={api_key}",
        symbols = symbols.join(",")
    );
    let resp = reqwest::get(&url).await?;
    let status = resp.status();
    if status == 200 {
        println!("OK");
        Ok(resp)
    } else {
        let data: Value = resp.json().await?;
        Err(anyhow::anyhow!(
            "request to {} returned status {}\nresponse: {:#?}",
            url,
            status,
            data
        ))
    }
}

#[derive(Debug, Deserialize)]
struct ShortQuote {
    symbol: String,
    price: f64,
}

#[derive(Debug, Deserialize)]
struct Quote {
    exchange: String,
    name: String,
    price: f64,
    symbol: String,
}

#[derive(Debug, Deserialize)]
struct PriceChange {
    #[serde(alias = "1D")]
    one_day: f64,
    symbol: String,
}

fn get_error(function: &str, symbols: &Vec<String>) -> String {
    format!("unable to get {} for {}", function, symbols.join(","))
}

pub async fn get_price(symbols: &Vec<String>) -> Result<()> {
    let resp = make_request("quote-short", symbols).await?;
    let results = resp
        .json::<Vec<ShortQuote>>()
        .await
        .with_context(|| get_error("price", symbols))?;
    for item in results {
        println!("Stock '{}' has price {}.", item.symbol, item.price);
    }
    Ok(())
}

pub async fn get_price_change(symbols: &Vec<String>) -> Result<()> {
    let resp = make_request("stock-price-change", symbols).await?;
    let results = resp
        .json::<Vec<PriceChange>>()
        .await
        .with_context(|| get_error("stock price change", symbols))?;
    for item in results {
        println!(
            "Stock {} has changed price by {}% over the course of 1 day.",
            item.symbol, item.one_day
        );
    }
    Ok(())
}

pub async fn get_quote(symbols: &Vec<String>) -> Result<()> {
    let resp = make_request("quote", symbols).await?;
    let results = resp
        .json::<Vec<Quote>>()
        .await
        .with_context(|| get_error("quote", symbols))?;
    for item in results {
        println!(
            "{} with symbol {} trading on {} has price of ${}.",
            item.name, item.symbol, item.exchange, item.price
        );
    }
    Ok(())
}
