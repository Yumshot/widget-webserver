use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error};

// Async function to fetch the current price of Bitcoin
pub async fn get_btc_price() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let env_api_key = env::var("XCOINAPIKEY").expect("XCOINAPIKEY not set in .env file");
    let url = "https://rest.coinapi.io/v1/quotes/BITSTAMP_SPOT_BTC_USD/current";

    let client = Client::new();
    let response: Value = client
        .get(url)
        .header("X-CoinAPI-Key", env_api_key)
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?; // Use the json() method here

    if let Some(last_trade_price) = response["last_trade"]["price"].as_f64() {
        Ok(last_trade_price.to_string())
    } else {
        Err("Failed to extract last_trade.price from the response.".into())
    }
}
