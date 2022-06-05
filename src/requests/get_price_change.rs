use crate::cli::Period;
use crate::requests::{errors, request};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_json::Value;

fn get_key(period: &Period) -> String {
    match period {
        Period::ALL => "ALL".to_string(),
        Period::D1 => "1D".to_string(),
        Period::D5 => "5D".to_string(),
        Period::M1 => "1M".to_string(),
        Period::M3 => "3M".to_string(),
        Period::M6 => "6M".to_string(),
        Period::YTD => "YTD".to_string(),
        Period::Y1 => "1Y".to_string(),
        Period::Y3 => "3Y".to_string(),
        Period::Y5 => "5Y".to_string(),
        Period::Y10 => "10".to_string(),
        Period::MAX => "MAX".to_string(),
    }
}

pub async fn get_price_change(symbols: &Vec<String>, period: &Period) -> Result<()> {
    let resp = request::make_request("stock-price-change", symbols).await?;
    let results: Vec<IndexMap<String, Value>> = resp
        .json()
        .await
        .with_context(|| errors::get_error("stock price change", symbols))?;
    for result in results {
        let symbol = &result["symbol"];
        match period {
            Period::ALL => {
                for (k, v) in (&result).into_iter().filter(|(x, _)| (*x).ne("symbol")) {
                    println!("{symbol} has changed price by {v}% over the course of {k}.");
                }
            }
            _ => {
                let period_key = get_key(period);
                for (k, v) in (&result)
                    .into_iter()
                    .filter(|(x, _)| x.to_uppercase().eq(&period_key))
                {
                    println!("{symbol} has changed price by {v}% over the course of {k}.");
                }
            }
        }
    }
    Ok(())
}
