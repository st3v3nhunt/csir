use anyhow::Result;
use reqwest::Response;
use serde_json::Value;
use std::env;

const URL: &str = "https://financialmodelingprep.com/api/v3";

pub async fn make_request(segment: &str, symbols: &Vec<String>) -> Result<Response> {
    let api_key = env::var("API_KEY")?;
    let url = format!(
        "{URL}/{segment}/{symbols}?apikey={api_key}",
        symbols = symbols.join(",")
    );
    let resp = reqwest::get(&url).await?;
    let status = resp.status();
    if status == 200 {
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
