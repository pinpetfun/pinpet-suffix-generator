use axum::{response::Json, http::StatusCode};
use crate::models::{ApiResponse, HealthResponse};

static START_TIME: std::sync::LazyLock<chrono::DateTime<chrono::Utc>> = 
    std::sync::LazyLock::new(|| chrono::Utc::now());

/// Health check endpoint
///
/// Check if the service is running normally, returns service status and uptime
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = ApiResponse<HealthResponse>),
        (status = 500, description = "Service error")
    ),
    tag = "Health Check"
)]
pub async fn health_check() -> Result<Json<ApiResponse<HealthResponse>>, StatusCode> {
    let uptime_duration = chrono::Utc::now().signed_duration_since(*START_TIME);
    let uptime = format!("{} days {} hours {} minutes", 
        uptime_duration.num_days(),
        uptime_duration.num_hours() % 24,
        uptime_duration.num_minutes() % 60
    );

    let health_data = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime,
    };

    Ok(Json(ApiResponse::success(health_data)))
}

/// Detailed health check
///
/// Returns more detailed system health status information
#[utoipa::path(
    get,
    path = "/health/detailed",
    responses(
        (status = 200, description = "Detailed health status", body = ApiResponse<serde_json::Value>),
    ),
    tag = "Health Check"
)]
pub async fn detailed_health_check() -> Json<ApiResponse<serde_json::Value>> {
    let memory_usage = get_memory_usage();
    let uptime_duration = chrono::Utc::now().signed_duration_since(*START_TIME);
    
    let detailed_info = serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": uptime_duration.num_seconds(),
        "memory_usage_kb": memory_usage,
        "timestamp": chrono::Utc::now(),
        "build_info": {
            "rust_version": option_env!("RUSTC_VERSION").unwrap_or("unknown"),
            "target": "unknown",
        }
    });

    Json(ApiResponse::success(detailed_info))
}

fn get_memory_usage() -> u64 {
    // Simple memory usage statistics, can use more professional libraries in production
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        return kb_str.parse().unwrap_or(0);
                    }
                }
            }
        }
    }
    0
}