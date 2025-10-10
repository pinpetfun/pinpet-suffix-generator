use std::env;

pub fn load_env() -> anyhow::Result<()> {
    // Try to load .env file
    if let Err(_) = dotenvy::dotenv() {
        tracing::warn!(".env file not found, using system environment variables");
    }
    Ok(())
}

pub fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}