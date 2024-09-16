//#![windows_subsystem = "windows"]
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use bitcoin::functions::get_btc_price;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use stocks::functions::check_stock_symbol;
use tokio::sync::Mutex;
use tokio::time::sleep;
use twitch::functions::check_if_channel_live;
use weather::functions::check_weather;

// Declare the bitcoin module
mod bitcoin {
    pub mod functions;
}

// Declare the twitch module
mod twitch {
    pub mod functions;
}

mod weather {
    pub mod functions;
}

mod stocks {
    pub mod functions;
}

// Shared state types for data and switch control
type SharedData = Arc<Mutex<String>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = Arc::new(Mutex::new(String::new()));
    let btc_price = Arc::new(Mutex::new(String::new())); // Variable to store Bitcoin price

    let data_clone = Arc::clone(&data);
    let btc_price_clone = Arc::clone(&btc_price);

    let mut task_index = 1; // Use a counter to rotate between tasks
    let mut btc_counter = 15; // Counter for 15-minute intervals

    // Background task for updating the data every minute
    tokio::spawn(async move {
        loop {
            match task_index {
                0 => {
                    // Check if we should query Bitcoin price (every 15 minutes)
                    if btc_counter >= 15 {
                        // Check Bitcoin Module
                        match get_btc_price().await {
                            Ok(price) => {
                                let mut btc_price_locked = btc_price_clone.lock().await;
                                *btc_price_locked = price.clone();
                                println!("Updated BTC data: ${}", price);
                                btc_counter = 0; // Reset the counter after fetching
                            }
                            Err(e) => eprintln!("Error fetching Bitcoin price: {}", e),
                        }
                    } else {
                        // Use the last stored Bitcoin price if within 15 minutes
                        let btc_price_locked = btc_price_clone.lock().await;
                        let mut data_locked = data_clone.lock().await;
                        *data_locked = format!("STATUS: ₿: ${}", *btc_price_locked);
                        println!("Using stored BTC data: ${}", *btc_price_locked);
                    }
                }
                1 => {
                    // Check Twitch Module
                    match check_if_channel_live().await {
                        Ok(status) => {
                            let mut data_locked = data_clone.lock().await;
                            *data_locked = format!("STATUS: MR K 💻: {}", status);
                            println!("Updated Twitch data");
                        }
                        Err(e) => eprintln!("Error checking Twitch live status: {}", e),
                    }
                }
                2 => {
                    // Check Weather Module
                    match check_weather().await {
                        Ok(result) => {
                            let mut data_locked = data_clone.lock().await;
                            *data_locked = format!("STATUS: 🌡️: {}°F", result);
                        }
                        Err(e) => eprintln!("Error in Weather function: {}", e),
                    }
                }
                3 => {
                    // Stock Symbol Module
                    match check_stock_symbol().await {
                        Ok(result) => {
                            let mut data_locked = data_clone.lock().await;
                            *data_locked = format!("STATUS: 🏛️S&P500: ${}", result);
                        }
                        Err(e) => eprintln!("Error in Stock Function: {}", e),
                    }
                }
                _ => (),
            }

            // Increment the counter for Bitcoin every minute
            if task_index == 0 {
                btc_counter += 1;
            }

            // Cycle through tasks
            task_index = (task_index + 1) % 3;

            // Sleep for a minute before checking again
            sleep(Duration::from_secs(60)).await;
        }
    });

    // Start the Actix web server with a single endpoint
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&data)))
            .service(single_endpoint)
    })
    .bind("0.0.0.0:8089")?
    .run()
    .await?;

    Ok(())
}

// Endpoint to serve the current data
#[actix_web::get("/current_data")]
async fn single_endpoint(data: web::Data<SharedData>) -> impl Responder {
    let data_locked = data.lock().await;
    println!("{:?}", data_locked);
    HttpResponse::Ok().body(format!("Current Info: {} end1", *data_locked))
}
