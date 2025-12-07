use anyhow::Result;
use pinpet_suffix_generator::{config::AppConfig, run_server};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = AppConfig::load()
        .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;

    // Run server
    run_server(config).await
}
