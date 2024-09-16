use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error};

// Async function to check if a Twitch channel is live
pub async fn check_if_channel_live() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    // Define client credentials
    let env_client_id = env::var("TWITCHCLIENTID").expect("TWITCHCLIENTID not set in .env file");
    let env_client_secret =
        env::var("TWITCHCLIENTSECRET").expect("TWITCHCLIENTSECRET not set in .env file");

    let channel_name = "lord_kebun";

    // Step 1: Get the OAuth token
    let token_url = format!(
        "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
        env_client_id, env_client_secret
    );

    let client = Client::new();
    let token_response: Value = client.post(&token_url).send().await?.json().await?; // Use the json() method here

    let access_token = token_response["access_token"]
        .as_str()
        .ok_or("Failed to extract access token")?;

    // Step 2: Check if the Twitch channel is live
    let check_url = format!(
        "https://api.twitch.tv/helix/streams?user_login={}",
        channel_name
    );

    let response: Value = client
        .get(&check_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", env_client_id)
        .send()
        .await?
        .json()
        .await?; // Use the json() method here

    // Check if the data array is empty (offline) or not (live)
    if let Some(data) = response["data"].as_array() {
        if data.is_empty() {
            // Channel is offline
            return Ok("⛔".to_string());
        } else {
            // Channel is live
            return Ok("✅".to_string());
        }
    }

    Err("Failed to parse Twitch API response".into())
}
