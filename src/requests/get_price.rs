use crate::requests::{errors, request};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ShortQuote {
    symbol: String,
    price: f64,
}

pub async fn get_price(symbols: &Vec<String>) -> Result<()> {
    let resp = request::make_request("quote-short", symbols).await?;
    let results = resp
        .json::<Vec<ShortQuote>>()
        .await
        .with_context(|| errors::get_error("price", symbols))?;
    for item in results {
        println!("{} has price {}.", item.symbol, item.price);
    }
    Ok(())
}
