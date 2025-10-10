use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub api: ApiConfig,
    pub logging: LoggingConfig,
    pub swagger: SwaggerConfig,
    pub pet_generator: PetGeneratorConfig,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiConfig {
    pub base_path: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SwaggerConfig {
    pub enabled: bool,
    pub path: String,
    pub title: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PetGeneratorConfig {
    pub pool_size: usize,
    pub batch_size: usize,
    pub db_path: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimitConfig {
    pub max_requests_per_minute: u32,
    pub window_seconds: u64,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".into());
        
        let builder = Config::builder()
            // Default configuration file
            .add_source(File::with_name("config").required(false))
            // Environment-specific configuration file
            .add_source(File::with_name(&format!("config.{}", env)).required(false))
            // Environment variable overrides with underscore separator
            .add_source(Environment::with_prefix("APP").separator("_"));

        builder.build()?.try_deserialize()
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub fn api_base_url(&self) -> String {
        format!("{}/{}", self.api.base_path, self.api.version)
    }
}