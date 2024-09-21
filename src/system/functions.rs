use std::error::Error;
use std::sync::Mutex;
use sysinfo::{ Networks, System };

// Create a static mutable index wrapped in a Mutex for thread safety
lazy_static::lazy_static! {
    static ref ROTATION_INDEX: Mutex<usize> = Mutex::new(0);
}

// Async function to check stock symbol
pub async fn gather_system_info() -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut targets = Vec::new();

    // Gather system info and add to targets vector
    targets.push(
        format!(
            "Total Memory: {} GB | Used Memory: {} GB",
            sys.total_memory() / 1024 / 1024 / 1024,
            sys.used_memory() / 1024 / 1024 / 1024
        )
    );
    targets.push(System::host_name().unwrap_or_default().to_string());

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        let builder = format!(
            "{}: {} MB (down) / {} MB (up)",
            interface_name,
            data.total_received() / 1024 / 1024,
            data.total_transmitted() / 1024 / 1024
        );
        targets.push(builder);
    }

    // Rotate the index instead of the vector
    let mut index = ROTATION_INDEX.lock().unwrap();
    let current_item = targets
        .get(*index % targets.len())
        .unwrap()
        .to_string();
    *index += 1; // Increment the index for the next call

    println!("{}", current_item);

    Ok(current_item)
}
