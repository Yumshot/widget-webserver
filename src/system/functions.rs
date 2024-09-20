use std::error::Error;
use sysinfo::{ Disks, Networks, System };

// Async function to check stock symbol
pub async fn gather_system_info() -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut targets = Vec::new();

    // Gather system info and add to targets vector
    targets.push(format!("Total Memory: {} MB", sys.total_memory() / 1024 / 1024));
    targets.push(format!("Used Memory: {} MB", sys.used_memory() / 1024 / 1024));
    targets.push(System::host_name().unwrap_or_default().to_string());

    // We display all disks' information:
    let disks = Disks::new_with_refreshed_list();
    if disks.len() > 1 {
        // Access the second disk
        targets.push(format!("Disk space: {:?}", disks[1].total_space() / 1024 / 1024 / 1024));
    }

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        let builder = format!(
            "{}: {} B (down) / {} B (up)",
            interface_name,
            data.total_received(),
            data.total_transmitted()
        );
        targets.push(builder);
    }

    // Rotate the first element to the back to change the first element on each call
    if !targets.is_empty() {
        // Moves the first element to the end, effectively rotating the list
        targets.rotate_left(1);
    }

    Ok(targets[0].to_string())
}
