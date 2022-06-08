use crate::cli::Period;
use crate::requests::{errors, request};
use ansi_term::Colour::Fixed;
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

fn price_change_colour(v: &Value) -> ansi_term::ANSIGenericString<str> {
    let v_i = v.as_f64().unwrap_or_else(|| f64::MAX);
    if v_i > 0. {
        // Bright Green
        Fixed(10).paint(v.to_string())
    } else if v_i < 0. {
        // Bright Red
        Fixed(9).paint(v.to_string())
    } else {
        // Bright Yellow
        Fixed(11).paint(v.to_string())
    }
}

fn print_price_change(symbol: &str, v: &Value, k: &str) {
    println!(
        "{} has changed price by {}% over the course of {}.",
        // Bright Blue
        Fixed(12).paint(symbol),
        price_change_colour(v),
        // Bright Magenta
        Fixed(13).paint(k)
    );
}

pub async fn get_price_change(symbols: &Vec<String>, period: &Period) -> Result<()> {
    let symbol_key = "symbol";
    let resp = request::make_request("stock-price-change", symbols).await?;
    let results: Vec<IndexMap<String, Value>> = resp
        .json()
        .await
        .with_context(|| errors::get_error("stock price change", symbols))?;
    for result in results {
        let symbol = &result[symbol_key].to_string();
        match period {
            Period::ALL => {
                for (k, v) in (&result).into_iter().filter(|(x, _)| (*x).ne(symbol_key)) {
                    print_price_change(symbol, v, k);
                }
            }
            _ => {
                let period_key = get_key(period);
                for (k, v) in (&result)
                    .into_iter()
                    .filter(|(x, _)| x.to_uppercase().eq(&period_key))
                {
                    print_price_change(symbol, v, k);
                }
            }
        }
    }
    Ok(())
}
