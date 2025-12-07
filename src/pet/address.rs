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
        const MAX_ATTEMPTS: usize = 10_000_000; // Limit attempts to avoid infinite loops
                                                 // Statistically need ~7,804 attempts on average for [a-z]Pet suffix

        for attempt in 1..=MAX_ATTEMPTS {
            let keypair = Keypair::new();
            let pubkey = keypair.pubkey();
            let address_str = pubkey.to_string();

            // Check if address ends with lowercase letter + "Pet" (e.g., aPet, bPet, zPet)
            if Self::is_valid_pet_suffix(&address_str) {
                return Some(Self {
                    public_key: pubkey.to_string(),
                    private_key: bs58::encode(&keypair.to_bytes()).into_string(),
                    address: address_str,
                });
            }

            // Log progress every 1M attempts
            if attempt % 1_000_000 == 0 {
                tracing::debug!("Pet address generation attempt {}/{}", attempt, MAX_ATTEMPTS);
            }
        }

        tracing::warn!("Failed to generate Pet address after {} attempts", MAX_ATTEMPTS);
        None
    }

    /// Validates that the address ends with a lowercase letter followed by "Pet"
    /// Valid examples: aPet, bPet, cPet, ..., zPet
    /// Invalid examples: APet, BPet, Pet, 1Pet
    fn is_valid_pet_suffix(address: &str) -> bool {
        if address.len() < 4 {
            return false;
        }

        let suffix = &address[address.len() - 4..];
        if !suffix.ends_with("Pet") {
            return false;
        }

        let first_char = suffix.chars().next().unwrap();
        first_char.is_ascii_lowercase()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pet_suffix() {
        // Valid lowercase letter + Pet suffixes
        assert!(PetAddress::is_valid_pet_suffix("aPet"));
        assert!(PetAddress::is_valid_pet_suffix("bPet"));
        assert!(PetAddress::is_valid_pet_suffix("zPet"));
        assert!(PetAddress::is_valid_pet_suffix("AGm9DpEaQYHxLKy98WGGoqErJEML9Pf5HySA1o4skPet"));
        assert!(PetAddress::is_valid_pet_suffix("SomeRandomAddressnPet"));
    }

    #[test]
    fn test_invalid_pet_suffix() {
        // Invalid: uppercase letter + Pet
        assert!(!PetAddress::is_valid_pet_suffix("APet"));
        assert!(!PetAddress::is_valid_pet_suffix("BPet"));
        assert!(!PetAddress::is_valid_pet_suffix("ZPet"));
        assert!(!PetAddress::is_valid_pet_suffix("AGm9DpEaQYHxLKy98WGGoqErJEML9Pf5HySA1o4sKPet"));

        // Invalid: just "Pet"
        assert!(!PetAddress::is_valid_pet_suffix("Pet"));

        // Invalid: number + Pet
        assert!(!PetAddress::is_valid_pet_suffix("1Pet"));
        assert!(!PetAddress::is_valid_pet_suffix("9Pet"));

        // Invalid: special character + Pet
        assert!(!PetAddress::is_valid_pet_suffix("!Pet"));
        assert!(!PetAddress::is_valid_pet_suffix("@Pet"));

        // Invalid: doesn't end with Pet
        assert!(!PetAddress::is_valid_pet_suffix("aPet1"));
        assert!(!PetAddress::is_valid_pet_suffix("test"));

        // Invalid: too short
        assert!(!PetAddress::is_valid_pet_suffix("abc"));
        assert!(!PetAddress::is_valid_pet_suffix(""));
    }
}