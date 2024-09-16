#![windows_subsystem = "windows"]

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::process::Command;
use std::str;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// The shared mutable state that will hold the Bitcoin price
type BtcPrice = Arc<Mutex<String>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Shared mutable variable to store the Bitcoin price
    let btc_price = Arc::new(Mutex::new(String::new()));

    // Clone the Arc to use inside the task that updates the price
    let btc_price_updater = Arc::clone(&btc_price);

    // Spawn a background task that updates the Bitcoin price every 15 minutes
    tokio::spawn(async move {
        loop {
            match get_btc_price() {
                Ok(price) => {
                    let mut btc_price_locked = btc_price_updater.lock().unwrap();
                    *btc_price_locked = price;
                    println!("Updated Bitcoin price: {}", *btc_price_locked);
                }
                Err(e) => eprintln!("Error fetching Bitcoin price: {}", e),
            }

            // Wait for 15 minutes before the next request
            sleep(Duration::from_secs(15 * 60)).await;
        }
    });

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&btc_price))) // Pass the shared state to the Actix web server
            .service(btc_price_endpoint) // Register the endpoint
    })
    .bind("0.0.0.0:8089")? // Bind to localhost:8089
    .run()
    .await?;

    Ok(())
}

#[actix_web::get("/btc_price")]
async fn btc_price_endpoint(btc_price: web::Data<BtcPrice>) -> impl Responder {
    // Lock the mutex to safely access the latest price
    let btc_price_locked = btc_price.lock().unwrap();

    // Return the latest Bitcoin price as a simple text response
    HttpResponse::Ok().body(format!("PRICE: ₿: ${} end1", *btc_price_locked))
}

// Function to fetch the current price of Bitcoin
fn get_btc_price() -> Result<String, Box<dyn Error>> {
    // Define the API endpoint and symbol ID for Bitcoin (BTC/USD)
    let url = "https://rest.coinapi.io/v1/quotes/BITSTAMP_SPOT_BTC_USD/current";
    dotenv().ok();
    let api_key = env::var("XCOINAPIKEY").expect("ANTHROPIC_API_KEY not set in .env file");

    // Construct the curl command to fetch the current price of Bitcoin
    let output = Command::new("curl")
        .arg("-L") // Follow redirects
        .arg(url) // API URL
        .arg("-H")
        .arg(format!("X-CoinAPI-Key: {}", api_key)) // API key header
        .arg("-H")
        .arg("Accept: application/json") // Set the Accept header to get JSON response
        .output()?; // Execute the command

    // Check if the command was successful
    if output.status.success() {
        // Parse the response as a UTF-8 string
        let response = str::from_utf8(&output.stdout)?;

        // Parse the response as JSON and extract the last trade price
        let json_response: serde_json::Value = serde_json::from_str(response)?;
        if let Some(last_trade_price) = json_response["last_trade"]["price"].as_f64() {
            // Return the last trade price as a string
            return Ok(last_trade_price.to_string());
        }

        Err("Failed to extract last_trade.price from the response.".into())
    } else {
        // If the request failed, return an error
        let error_message = str::from_utf8(&output.stderr)?;
        Err(format!("Failed to fetch Bitcoin price: {}", error_message).into())
    }
}
