use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ServerTimeResponse {
    /// Current server time (UTC)
    #[schema(example = "2024-01-15T10:30:00Z")]
    pub current_time: DateTime<Utc>,
    /// Timestamp (seconds)
    #[schema(example = 1705316200)]
    pub timestamp: i64,
    /// Timezone information
    #[schema(example = "UTC")]
    pub timezone: String,
    /// Formatted local time
    #[schema(example = "2024-01-15 10:30:00")]
    pub formatted: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TimeQuery {
    /// Return format (iso8601, timestamp, formatted)
    #[schema(example = "iso8601")]
    pub format: Option<String>,
    /// Timezone offset (e.g.: +08:00)
    #[schema(example = "+08:00")]
    pub timezone: Option<String>,
}

impl ServerTimeResponse {
    pub fn new(query: &TimeQuery) -> Self {
        let now = Utc::now();
        
        let formatted = match query.format.as_deref() {
            Some("timestamp") => now.timestamp().to_string(),
            Some("formatted") => now.format("%Y-%m-%d %H:%M:%S").to_string(),
            _ => now.to_rfc3339(),
        };

        Self {
            current_time: now,
            timestamp: now.timestamp(),
            timezone: "UTC".to_string(),
            formatted,
        }
    }
}