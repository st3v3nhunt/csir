use anyhow::{Context, Result};
use serde::Deserialize;

use crate::request;

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
        .with_context(|| request::get_error("price", symbols))?;
    for item in results {
        println!("Stock '{}' has price {}.", item.symbol, item.price);
    }
    Ok(())
}
