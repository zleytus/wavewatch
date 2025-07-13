mod config;
mod error;
mod rf_explorer;
mod server_client;

use clap::Parser;
use config::Config;
use error::Result;
use log::{error, info};
use rf_explorer::RfExplorerDevice;
use server_client::ServerClient;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config = Config::parse();

    if config.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    info!("Starting Wavewatch RF Explorer Client");
    info!("Server URL: {}", config.server_url);

    if let Err(e) = run_client(config).await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run_client(config: Config) -> Result<()> {
    let server_client = ServerClient::new(config.server_url.clone());

    let mut rf_explorer = RfExplorerDevice::connect(config.device_port.clone()).await?;

    let analyzer_id = rf_explorer.get_analyzer_id().to_string();
    info!("Using analyzer ID: {}", analyzer_id);

    rf_explorer.configure(config.start_freq, config.stop_freq)?;

    info!("Starting real-time data collection. Press Ctrl+C to stop.");

    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        // Check if Ctrl+C was pressed
        if !running.load(Ordering::SeqCst) {
            info!("Received shutdown signal, stopping...");
            break;
        }

        match rf_explorer.get_trace() {
            Ok(trace_data) => {
                if let Err(e) = server_client
                    .send_trace_with_retry(&analyzer_id, &trace_data, 3)
                    .await
                {
                    error!("Failed to send trace data: {}", e);
                }
            }
            Err(e) => {
                error!("Error getting trace from RF Explorer: {}", e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    info!("Wavewatch RF Explorer Client stopped");
    Ok(())
}
