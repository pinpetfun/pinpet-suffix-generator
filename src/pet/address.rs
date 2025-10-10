use serde::{Deserialize, Serialize};
use solana_sdk::signature::{Keypair, Signer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetAddress {
    pub public_key: String,
    pub private_key: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetAddressInfo {
    pub id: u64,
    pub address: PetAddress,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl PetAddress {
    pub fn generate() -> Option<Self> {
        const MAX_ATTEMPTS: usize = 1_000_000; // Limit attempts to avoid infinite loops
        
        for attempt in 1..=MAX_ATTEMPTS {
            let keypair = Keypair::new();
            let pubkey = keypair.pubkey();
            let address_str = pubkey.to_string();
            
            if address_str.ends_with("Pet") {
                return Some(Self {
                    public_key: pubkey.to_string(),
                    private_key: bs58::encode(&keypair.to_bytes()).into_string(),
                    address: address_str,
                });
            }
            
            // Log progress every 100k attempts
            if attempt % 100_000 == 0 {
                tracing::debug!("Pet address generation attempt {}/{}", attempt, MAX_ATTEMPTS);
            }
        }
        
        tracing::warn!("Failed to generate Pet address after {} attempts", MAX_ATTEMPTS);
        None
    }
    
    pub fn from_keypair(keypair: &Keypair) -> Self {
        let pubkey = keypair.pubkey();
        Self {
            public_key: pubkey.to_string(),
            private_key: bs58::encode(&keypair.to_bytes()).into_string(),
            address: pubkey.to_string(),
        }
    }
    
    pub fn to_keypair(&self) -> Result<Keypair, Box<dyn std::error::Error>> {
        let private_key_bytes = bs58::decode(&self.private_key).into_vec()?;
        Ok(Keypair::try_from(&private_key_bytes[..])?)
    }
}