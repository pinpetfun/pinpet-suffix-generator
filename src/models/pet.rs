use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetPetAddressResponse {
    pub id: u64,
    pub public_key: String,
    pub private_key: String,
    pub address: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PetGeneratorStatusResponse {
    pub total_addresses: usize,
    pub pool_size: usize,
    pub generation_active: bool,
}