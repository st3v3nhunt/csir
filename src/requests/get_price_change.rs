use crate::requests::{errors, request};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PriceChange {
    #[serde(alias = "1D")]
    one_day: f64,
    symbol: String,
}

pub async fn get_price_change(symbols: &Vec<String>) -> Result<()> {
    let resp = request::make_request("stock-price-change", symbols).await?;
    let results = resp
        .json::<Vec<PriceChange>>()
        .await
        .with_context(|| errors::get_error("stock price change", symbols))?;
    for item in results {
        println!(
            "Stock {} has changed price by {}% over the course of 1 day.",
            item.symbol, item.one_day
        );
    }
    Ok(())
}
