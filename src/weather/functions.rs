use chrono::{Local, Timelike};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error};

pub async fn check_weather() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let env_coords_x = env::var("WEATHERX").expect("WEATHERX not set in .env file");
    let env_coords_y = env::var("WEATHERY").expect("WEATHERY not set in .env file");
    // API endpoint with parameters
    let formatted_coords = format!("latitude={}&longitude={}", env_coords_x, env_coords_y);
    let base_url = format!("https://api.open-meteo.com/v1/forecast?{}&hourly=temperature_2m&temperature_unit=fahrenheit&wind_speed_unit=mph&precipitation_unit=inch&timezone=America%2FLos_Angeles&forecast_days=1", formatted_coords);

    // Create a new HTTP client
    let client = Client::new();

    // Send the GET request
    let response = client
        .get(base_url) // Make GET request to the URL
        .send() // Send the request
        .await? // Await the response (this is an async operation)
        .text() // Get the response body as text
        .await?; // Await the conversion to text

    // Parse the response text as JSON
    let json_response: Value = serde_json::from_str(&response)?;

    // Get the hourly temperatures
    let target_temps = &json_response["hourly"]["temperature_2m"];

    // Get the current hour and cast it to usize
    let now = Local::now();
    let current_hour = now.hour() as usize;

    // Safely access the temperature for the current hour
    if let Some(callback) = target_temps.get(current_hour) {
        // Return the temperature as a formatted string
        Ok(format!("{}", callback))
    } else {
        // Return a message if there's no temperature data for the current hour
        Ok("No temperature data available for the current hour.".to_string())
    }
}
