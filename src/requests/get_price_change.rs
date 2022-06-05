use crate::cli::Period;
use crate::requests::{errors, request};
use anyhow::{Context, Result};
use serde_json::Value;

struct PeriodInfo {
    key: String,
    phrase: String,
}

fn get_info(period: &Period) -> PeriodInfo {
    match period {
        Period::D1 => PeriodInfo {
            key: "1D".to_string(),
            phrase: "1 day".to_string(),
        },
        Period::D5 => PeriodInfo {
            key: "5D".to_string(),
            phrase: "5 days".to_string(),
        },
        Period::M1 => PeriodInfo {
            key: "M1".to_string(),
            phrase: "1 month".to_string(),
        },
    }
}

pub async fn get_price_change(symbols: &Vec<String>, period: &Period) -> Result<()> {
    let resp = request::make_request("stock-price-change", symbols).await?;
    let results: Vec<Value> = resp
        .json()
        .await
        .with_context(|| errors::get_error("stock price change", symbols))?;
    let info = get_info(period);
    for item in results {
        println!(
            "{} has changed price by {}% over the course of {}.",
            item["symbol"], item[&info.key], &info.phrase
        );
    }
    Ok(())
}
