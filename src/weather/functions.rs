use chrono::{Local, Timelike};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error};

pub async fn check_weather() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    // Load coordinates from environment
    let latitude = env::var("WEATHERX")?;
    let longitude = env::var("WEATHERY")?;

    // Construct the API URL with the coordinates
    let api_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m&temperature_unit=fahrenheit&wind_speed_unit=mph&precipitation_unit=inch&timezone=America%2FLos_Angeles&forecast_days=1",
        latitude, longitude
    );

    // Create a new HTTP client
    let client = Client::new();

    // Send the GET request and parse the response as JSON
    let response: Value = client.get(&api_url).send().await?.json().await?;

    // Extract the hourly temperatures
    let temperatures = &response["hourly"]["temperature_2m"];

    // Get the current hour as usize
    let current_hour = Local::now().hour() as usize;

    // Safely access and return the temperature for the current hour
    if let Some(temperature) = temperatures.get(current_hour) {
        Ok(format!("{}", temperature))
    } else {
        Ok("No temperature data available for the current hour.".to_string())
    }
}
