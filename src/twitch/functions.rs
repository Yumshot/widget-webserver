use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error};

pub async fn check_if_channel_live() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    let client_id = env::var("TWITCHCLIENTID")?;
    let client_secret = env::var("TWITCHCLIENTSECRET")?;
    let channel_name = "lord_kebun";

    // Step 1: Get the OAuth token
    let token_url = format!(
        "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
        client_id, client_secret
    );

    let client = Client::new();
    let token_response: Value = client.post(&token_url).send().await?.json().await?;

    let access_token = match token_response["access_token"].as_str() {
        Some(token) => token,
        None => return Err("Failed to extract access token".into()),
    };

    // Step 2: Check if the Twitch channel is live
    let check_url = format!(
        "https://api.twitch.tv/helix/streams?user_login={}",
        channel_name
    );

    let response: Value = client
        .get(&check_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", &client_id)
        .send()
        .await?
        .json()
        .await?;

    // Check if the data array contains stream information
    let is_live = response["data"]
        .as_array()
        .map_or(false, |data| !data.is_empty());

    Ok(if is_live {
        "✅".to_string()
    } else {
        "⛔".to_string()
    })
}
