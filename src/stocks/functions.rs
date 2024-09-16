use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error};

// Async function to check stock symbol
pub async fn check_stock_symbol() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    // Load the API key from the environment
    let api_key = env::var("FINNHUBAPI")?;
    let symbol = "SPY"; // You can replace this with any symbol
    let request_url = format!(
        "https://finnhub.io/api/v1/quote?symbol={}&token={}",
        symbol, api_key
    );

    // Create a new HTTP client
    let client = Client::new();

    // Send a GET request to the Finnhub API and get the response as JSON
    let response: Value = client.get(&request_url).send().await?.json().await?;

    // Extract and return the "c" field (current price)
    response["c"]
        .as_f64()
        .map(|price| format!("{:.2}", price))
        .ok_or_else(|| "Failed to extract the current price.".into())
}
