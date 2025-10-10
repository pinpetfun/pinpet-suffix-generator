use axum::{
    extract::Query,
    response::Json,
    http::StatusCode,
};
use crate::models::{ApiResponse, ServerTimeResponse, TimeQuery};

/// Get current server time
///
/// Returns the current server time, supports different format parameters and timezone settings
#[utoipa::path(
    get,
    path = "/api/v1/time",
    params(
        ("format" = Option<String>, Query, description = "Time format (iso8601, timestamp, formatted)", example = "iso8601"),
        ("timezone" = Option<String>, Query, description = "Timezone offset (e.g.: +08:00)", example = "+08:00")
    ),
    responses(
        (status = 200, description = "Successfully returned server time", body = ApiResponse<ServerTimeResponse>),
        (status = 400, description = "Request parameter error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Service"
)]
pub async fn get_server_time(
    Query(query): Query<TimeQuery>
) -> Result<Json<ApiResponse<ServerTimeResponse>>, StatusCode> {
    
    // Validate time format parameter
    if let Some(ref format) = query.format {
        match format.as_str() {
            "iso8601" | "timestamp" | "formatted" => {},
            _ => {
                let error_response: ApiResponse<ServerTimeResponse> = ApiResponse {
                    code: 400,
                    message: "Invalid time format, supported formats: iso8601, timestamp, formatted".to_string(),
                    data: None,
                    timestamp: chrono::Utc::now().timestamp(),
                };
                return Ok(Json(error_response));
            }
        }
    }
    
    let response = ServerTimeResponse::new(&query);
    Ok(Json(ApiResponse::success(response)))
}

/// Get multi-timezone time
///
/// Returns current time in multiple timezones
#[utoipa::path(
    get,
    path = "/api/v1/time/zones",
    responses(
        (status = 200, description = "Successfully returned multi-timezone time", body = ApiResponse<serde_json::Value>),
    ),
    tag = "Time Service"
)]
pub async fn get_multi_timezone() -> Json<ApiResponse<serde_json::Value>> {
    let now = chrono::Utc::now();
    
    let timezones = serde_json::json!({
        "utc": {
            "time": now,
            "timestamp": now.timestamp(),
            "formatted": now.format("%Y-%m-%d %H:%M:%S UTC").to_string()
        },
        "beijing": {
            "time": now + chrono::Duration::hours(8),
            "timestamp": now.timestamp(),
            "formatted": (now + chrono::Duration::hours(8)).format("%Y-%m-%d %H:%M:%S +08:00").to_string()
        },
        "tokyo": {
            "time": now + chrono::Duration::hours(9),
            "timestamp": now.timestamp(),
            "formatted": (now + chrono::Duration::hours(9)).format("%Y-%m-%d %H:%M:%S +09:00").to_string()
        },
        "new_york": {
            "time": now - chrono::Duration::hours(5),
            "timestamp": now.timestamp(),
            "formatted": (now - chrono::Duration::hours(5)).format("%Y-%m-%d %H:%M:%S -05:00").to_string()
        },
        "london": {
            "time": now,
            "timestamp": now.timestamp(),
            "formatted": now.format("%Y-%m-%d %H:%M:%S +00:00").to_string()
        }
    });

    Json(ApiResponse::success(timezones))
}