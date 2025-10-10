use tower_http::cors::{CorsLayer, AllowOrigin, AllowMethods, AllowHeaders};
use axum::http::{Method, HeaderName};

#[cfg(feature = "production")]
use axum::http::HeaderValue;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        // Allow all origins - development environment configuration
        .allow_origin(AllowOrigin::any())
        // Allowed HTTP methods
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
        ]))
        // Allowed request headers
        .allow_headers(AllowHeaders::list([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::ACCEPT_LANGUAGE,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::CONTENT_LENGTH,
            axum::http::header::ORIGIN,
            axum::http::header::USER_AGENT,
            HeaderName::from_static("x-requested-with"),
            // Custom headers
            HeaderName::from_static("x-request-id"),
            HeaderName::from_static("x-api-key"),
        ]))
        // Exposed response headers
        .expose_headers([
            axum::http::header::CONTENT_LENGTH,
            axum::http::header::CONTENT_TYPE,
        ])
        // No credentials needed for development environment
        // Preflight request cache time
        .max_age(std::time::Duration::from_secs(3600))
}

#[cfg(feature = "production")]
pub fn production_cors_layer(allowed_origins: Vec<String>) -> CorsLayer {
    let origins: Vec<HeaderValue> = allowed_origins
        .into_iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();
    
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
            HeaderName::from_static("x-api-key"),
        ]))
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(86400))
}