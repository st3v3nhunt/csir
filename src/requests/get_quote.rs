use crate::requests::{errors, request};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Quote {
    exchange: String,
    name: String,
    price: f64,
    symbol: String,
}

pub async fn get_quote(symbols: &Vec<String>) -> Result<()> {
    let resp = request::make_request("quote", symbols).await?;
    let results = resp
        .json::<Vec<Quote>>()
        .await
        .with_context(|| errors::get_error("quote", symbols))?;
    for item in results {
        println!(
            "{} with symbol {} trading on {} has price of ${}.",
            item.name, item.symbol, item.exchange, item.price
        );
    }
    Ok(())
}
