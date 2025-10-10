use axum::{routing::get, Router};
use std::sync::Arc;
use crate::handlers::{health_check, detailed_health_check, get_server_time, get_multi_timezone, get_pet_address, get_pet_status, PetAppState};
use crate::config::AppConfig;

pub fn health_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/detailed", get(detailed_health_check))
}

pub fn time_routes() -> Router {
    Router::new()
        .route("/time", get(get_server_time))
        .route("/time/zones", get(get_multi_timezone))
}

pub fn pet_routes() -> Router<Arc<PetAppState>> {
    Router::new()
        .route("/pet/address", get(get_pet_address))
}

pub fn pet_status_routes() -> Router<Arc<PetAppState>> {
    Router::new()
        .route("/pet/status", get(get_pet_status))
}

pub fn api_routes(config: &AppConfig) -> (Router, Router<Arc<PetAppState>>, Router<Arc<PetAppState>>) {
    let api_prefix = &config.api_base_url();
    
    let time_api = Router::new().nest(api_prefix, time_routes());
    let pet_api = Router::new().nest(api_prefix, pet_routes());
    let pet_status_api = Router::new().nest(api_prefix, pet_status_routes());
    
    (time_api, pet_api, pet_status_api)
}

pub fn create_routes(config: &AppConfig) -> (Router, Router<Arc<PetAppState>>, Router<Arc<PetAppState>>) {
    let (time_api, pet_api, pet_status_api) = api_routes(config);
    
    let base_routes = Router::new()
        .merge(health_routes())
        .merge(time_api);
    
    (base_routes, pet_api, pet_status_api)
}