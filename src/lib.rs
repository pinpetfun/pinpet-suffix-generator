pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;
pub mod pet;

use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Arc;

use crate::config::AppConfig;
use crate::middleware::{cors_layer, logging_layer, RateLimiter, rate_limit_middleware};
use crate::routes::create_routes;
use crate::handlers::PetAppState;
use crate::pet::{PetGenerator, PetStorage};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_check,
        crate::handlers::health::detailed_health_check,
        crate::handlers::time::get_server_time,
        crate::handlers::time::get_multi_timezone,
        crate::handlers::pet::get_pet_address,
        crate::handlers::pet::get_pet_status,
    ),
    components(schemas(
        crate::models::ApiResponse<crate::models::HealthResponse>,
        crate::models::ApiResponse<crate::models::ServerTimeResponse>,
        crate::models::ApiResponse<crate::models::GetPetAddressResponse>,
        crate::models::ApiResponse<crate::models::PetGeneratorStatusResponse>,
        crate::models::ApiResponse<serde_json::Value>,
        crate::models::HealthResponse,
        crate::models::ServerTimeResponse,
        crate::models::GetPetAddressResponse,
        crate::models::PetGeneratorStatusResponse,
        crate::models::TimeQuery,
    )),
    tags(
        (name = "Time Service", description = "APIs for getting server time"),
        (name = "Health Check", description = "Service health status check"),
        (name = "Pet Address", description = "APIs for Pet address generation and management")
    ),
    info(
        title = "PetAddr Server API",
        description = "API documentation for PetAddr server project - designed with layered architecture",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;

pub async fn create_app(config: AppConfig) -> anyhow::Result<(Router, Arc<PetGenerator>)> {
    // Initialize Pet storage
    let storage = Arc::new(PetStorage::new(&config.pet_generator.db_path)?);
    
    // Initialize Pet generator
    let generator = Arc::new(PetGenerator::new(
        Arc::clone(&storage),
        config.pet_generator.clone(),
    ));
    
    // Create Pet app state
    let pet_state = Arc::new(PetAppState {
        generator: Arc::clone(&generator),
        storage,
    });
    
    // Create rate limiter
    let rate_limiter = RateLimiter::new(
        config.rate_limit.max_requests_per_minute,
        config.rate_limit.window_seconds,
    );
    
    let (base_routes, pet_routes, pet_status_routes) = create_routes(&config);
    
    let mut app = Router::new()
        .merge(base_routes)
        .merge(pet_status_routes.with_state(Arc::clone(&pet_state)))
        .merge(pet_routes.with_state(pet_state));

    // Add Swagger UI if enabled
    if config.swagger.enabled {
        let swagger_path = config.swagger.path.clone();
        let swagger_url = format!("{}/openapi.json", swagger_path.trim_end_matches('/'));
        app = app.merge(
            SwaggerUi::new(swagger_path)
                .url(swagger_url, ApiDoc::openapi())
        );
    }

    // Add middleware layers
    app = app.layer(
        ServiceBuilder::new()
            .layer(logging_layer())
            .layer(cors_layer())
    );

    Ok((app, generator))
}

pub async fn run_server(config: AppConfig) -> anyhow::Result<()> {
    // Initialize logging
    init_logging(&config.logging.level);

    // Create database directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&config.pet_generator.db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let (app, generator) = create_app(config.clone()).await?;
    
    // Start Pet address generator
    generator.start().await?;
    
    let addr = config.server_address();
    
    tracing::info!("🚀 Server started successfully!");
    tracing::info!("📡 Listening on: http://{}", addr);
    
    if config.swagger.enabled {
        tracing::info!("📊 API Documentation: http://{}{}", addr, config.swagger.path);
    }
    
    tracing::info!("⏰ Time API: http://{}{}/time", addr, config.api_base_url());
    tracing::info!("🐕 Pet Address API: http://{}{}/pet/address", addr, config.api_base_url());
    tracing::info!("📊 Pet Status API: http://{}{}/pet/status", addr, config.api_base_url());
    tracing::info!("❤️  Health Check: http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await?;
    
    Ok(())
}

fn init_logging(level: &str) {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();
}