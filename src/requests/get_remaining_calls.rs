use ansi_term::Colour::Fixed;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct RemainingCallsResult {
    result: i16,
}

pub async fn get_remaining_calls() -> Result<()> {
    let api_key = env::var("API_KEY")?;
    let json = serde_json::json!({
        "data": {
            "key": api_key
        }
    });

    let host = env::var("REMAINING_CALLS_API_URL")?;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{host}/getRemainingCalls"))
        .json(&json)
        .send()
        .await
        .with_context(|| "unable to get remaining calls")?;
    let result: RemainingCallsResult = resp
        .json()
        .await
        .with_context(|| "unable to parse response body")?;
    println!(
        "API key '{}' has {} calls remaining.",
        Fixed(13).paint(api_key),
        Fixed(9).paint(result.result.to_string())
    );
    Ok(())
}
