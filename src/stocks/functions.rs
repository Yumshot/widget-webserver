use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error};

// Async function to check stock symbol
pub async fn check_stock_symbol() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    // Load the API key from the environment
    let env_stock_api_key = env::var("FINNHUBAPI").expect("FINNHUBAPI not set in .env file");
    let symbol = "SPY"; // You can replace this with any symbol
    let request_url = format!(
        "https://finnhub.io/api/v1/quote?symbol={}&token={}",
        symbol, env_stock_api_key
    );

    // Create a new HTTP client
    let client = Client::new();

    // Send a GET request to the Finnhub API
    let response = client.get(&request_url).send().await?.text().await?;

    // Parse the response text as JSON
    let json_response: Value = serde_json::from_str(&response)?;

    // Extract the "c" field (current price)
    if let Some(current_price) = json_response["c"].as_f64() {
        // Return the current price as a formatted string
        return Ok(format!("{:.2}", current_price));
    } else {
        // If the "c" field is not found, return an error
        return Err("Response Failed.".into());
    }
}
