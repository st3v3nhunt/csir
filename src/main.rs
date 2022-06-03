use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Default, Parser)]
#[clap(author = "st3v3nhunt", version, about)]
/// CLI Sotck Info Retriever
struct Args {
    #[clap(forbid_empty_values = true)]
    /// Symbol to retrieve info about
    symbol: String,
}

#[derive(Deserialize, Debug)]
struct ShortQuote {
    symbol: String,
    price: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let args = Args::parse();
    println!("{:?}", args);

    let result = get_quote(&api_key, &args.symbol).await?;
    match result {
        Some(r) => println!("Stock '{}' has price {}.", r.symbol, r.price),
        None => println!("No result returned for '{}'", args.symbol),
    }
    Ok(())
}

async fn get_quote(api_key: &str, symbol: &str) -> Result<Option<ShortQuote>> {
    let resp = reqwest::get(format!(
        "https://financialmodelingprep.com/api/v3/quote-short/{symbol}?apikey={api_key}",
        symbol = symbol,
        api_key = api_key
    ))
    .await?;
    match resp.status() {
        reqwest::StatusCode::OK => {
            let data = resp.json::<Vec<ShortQuote>>().await?;
            println!("Response data: {:?}", data);
            Ok(data.into_iter().nth(0))
        }
        _ => {
            println!(
                "Response for '{}' returned status code '{}' and was not parsable.",
                symbol,
                resp.status().as_str()
            );
            Ok(None)
        }
    }
}
