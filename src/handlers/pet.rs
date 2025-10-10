use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::models::{ApiResponse, GetPetAddressResponse, PetGeneratorStatusResponse};
use crate::pet::{PetGenerator, PetStorage};

pub struct PetAppState {
    pub generator: Arc<PetGenerator>,
    pub storage: Arc<PetStorage>,
}

#[utoipa::path(
    get,
    path = "/api/v1/pet/address",
    responses(
        (status = 200, description = "Successfully retrieved Pet address", body = ApiResponse<GetPetAddressResponse>),
        (status = 404, description = "No Pet addresses available", body = ApiResponse<String>),
        (status = 500, description = "Internal server error", body = ApiResponse<String>)
    ),
    tag = "Pet Address"
)]
pub async fn get_pet_address(
    State(app_state): State<Arc<PetAppState>>,
) -> Result<Json<ApiResponse<GetPetAddressResponse>>, StatusCode> {
    match app_state.storage.get_next_address() {
        Ok(Some(address_info)) => {
            let response = GetPetAddressResponse {
                id: address_info.id,
                public_key: address_info.address.public_key,
                private_key: address_info.address.private_key,
                address: address_info.address.address,
                created_at: address_info.created_at.to_rfc3339(),
            };
            
            Ok(Json(ApiResponse::success(response)))
        }
        Ok(None) => {
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            tracing::error!("Failed to get Pet address: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/pet/status",
    responses(
        (status = 200, description = "Pet generator status", body = ApiResponse<PetGeneratorStatusResponse>),
        (status = 500, description = "Internal server error", body = ApiResponse<String>)
    ),
    tag = "Pet Address"
)]
pub async fn get_pet_status(
    State(app_state): State<Arc<PetAppState>>,
) -> Result<Json<ApiResponse<PetGeneratorStatusResponse>>, StatusCode> {
    match app_state.generator.get_current_count().await {
        Ok(count) => {
            let response = PetGeneratorStatusResponse {
                total_addresses: count,
                pool_size: 100, // TODO: Get from config
                generation_active: true, // TODO: Get actual status
            };
            
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get Pet generator status: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}