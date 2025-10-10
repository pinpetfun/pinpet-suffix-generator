use anyhow::{Result, Context};
use sled::Db;
use std::path::Path;

use super::address::{PetAddress, PetAddressInfo};

#[derive(Clone)]
pub struct PetStorage {
    db: Db,
    counter_key: &'static str,
}

impl PetStorage {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db = sled::open(db_path)?;
        Ok(Self {
            db,
            counter_key: "counter",
        })
    }
    
    pub fn store_address(&self, address: PetAddress) -> Result<u64> {
        let id = self.next_id()?;
        let address_info = PetAddressInfo {
            id,
            address,
            created_at: chrono::Utc::now(),
        };
        
        let key = format!("address:{:010}", id);
        let value = serde_json::to_vec(&address_info)
            .context("Failed to serialize address info")?;
        
        self.db.insert(key.as_bytes(), value)?;
        self.db.flush()?;
        
        Ok(id)
    }
    
    pub fn get_next_address(&self) -> Result<Option<PetAddressInfo>> {
        for result in self.db.scan_prefix(b"address:") {
            let (_key, value) = result?;
            let address_info: PetAddressInfo = serde_json::from_slice(&value)
                .context("Failed to deserialize address info")?;
            
            // Remove this address from storage after retrieving
            let key = format!("address:{:010}", address_info.id);
            self.db.remove(key.as_bytes())?;
            
            return Ok(Some(address_info));
        }
        
        Ok(None)
    }
    
    pub fn count_addresses(&self) -> Result<usize> {
        let count = self.db.scan_prefix(b"address:").count();
        Ok(count)
    }
    
    pub fn clear_all_addresses(&self) -> Result<()> {
        let keys: Vec<_> = self.db.scan_prefix(b"address:")
            .map(|result| result.unwrap().0)
            .collect();
        
        for key in keys {
            self.db.remove(&key)?;
        }
        
        self.db.flush()?;
        Ok(())
    }
    
    fn next_id(&self) -> Result<u64> {
        let id = self.db
            .update_and_fetch(self.counter_key, |old| {
                let current = old
                    .map(|bytes| {
                        let mut array = [0u8; 8];
                        array.copy_from_slice(bytes);
                        u64::from_be_bytes(array)
                    })
                    .unwrap_or(0);
                Some((current + 1).to_be_bytes().to_vec())
            })?
            .context("Failed to update counter")?;
        
        let mut array = [0u8; 8];
        array.copy_from_slice(&id);
        Ok(u64::from_be_bytes(array))
    }
}