use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    /// Status code
    #[schema(example = 200)]
    pub code: i32,
    /// Response message
    #[schema(example = "success")]
    pub message: String,
    /// Response data
    pub data: Option<T>,
    /// Request timestamp
    #[schema(example = 1705316200)]
    pub timestamp: i64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    /// Service status
    #[schema(example = "healthy")]
    pub status: String,
    /// Service version
    #[schema(example = "1.0.0")]
    pub version: String,
    /// Service uptime
    #[schema(example = "2024-01-15T10:30:00Z")]
    pub uptime: String,
}