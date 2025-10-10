use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

use crate::config::PetGeneratorConfig;
use super::address::PetAddress;
use super::storage::PetStorage;

pub struct PetGenerator {
    storage: Arc<PetStorage>,
    config: PetGeneratorConfig,
    is_running: Arc<Mutex<bool>>,
}

impl PetGenerator {
    pub fn new(storage: Arc<PetStorage>, config: PetGeneratorConfig) -> Self {
        Self {
            storage,
            config,
            is_running: Arc::new(Mutex::new(false)),
        }
    }
    
    pub async fn start(&self) -> Result<()> {
        {
            let mut running = self.is_running.lock().await;
            if *running {
                return Ok(());
            }
            *running = true;
        }
        
        info!("Starting Pet address generator");
        
        let storage = Arc::clone(&self.storage);
        let config = self.config.clone();
        let is_running = Arc::clone(&self.is_running);
        
        tokio::spawn(async move {
            loop {
                {
                    let running = is_running.lock().await;
                    if !*running {
                        break;
                    }
                }
                
                match storage.count_addresses() {
                    Ok(count) => {
                        if count < config.pool_size {
                            let need_to_generate = config.pool_size - count;
                            let batch_size = std::cmp::min(need_to_generate, config.batch_size);
                            
                            info!("Current address count: {}, generating {} more addresses", count, batch_size);
                            
                            Self::generate_batch(&storage, batch_size).await;
                        }
                    }
                    Err(e) => {
                        error!("Failed to check address count: {}", e);
                    }
                }
                
                sleep(Duration::from_secs(5)).await;
            }
            
            info!("Pet address generator stopped");
        });
        
        Ok(())
    }
    
    pub async fn stop(&self) {
        let mut running = self.is_running.lock().await;
        *running = false;
        info!("Stopping Pet address generator");
    }
    
    async fn generate_batch(storage: &PetStorage, count: usize) {
        let (tx, mut rx) = mpsc::channel(count);
        
        // Spawn generation tasks
        for i in 0..count {
            let tx = tx.clone();
            tokio::spawn(async move {
                info!("Starting generation task {}", i + 1);
                
                // Retry up to 3 times if generation fails
                for retry in 1..=3 {
                    match PetAddress::generate() {
                        Some(address) => {
                            info!("Generated Pet address ending with: {}", 
                                  &address.address[address.address.len().saturating_sub(10)..]);
                            if tx.send(address).await.is_err() {
                                warn!("Failed to send generated address to channel");
                            }
                            break; // Success, exit retry loop
                        }
                        None => {
                            warn!("Failed to generate Pet address in task {} (attempt {}/3)", i + 1, retry);
                            if retry < 3 {
                                // Wait a bit before retrying
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                        }
                    }
                }
            });
        }
        
        drop(tx); // Close the sender
        
        // Collect generated addresses with timeout
        let mut generated_count = 0;
        let timeout_duration = Duration::from_secs(30); // 30 second timeout per batch
        let start_time = std::time::Instant::now();
        
        while let Some(address) = rx.recv().await {
            if start_time.elapsed() > timeout_duration {
                warn!("Batch generation timed out after 30 seconds");
                break;
            }
            
            match storage.store_address(address) {
                Ok(id) => {
                    generated_count += 1;
                    info!("Stored Pet address with ID: {}", id);
                }
                Err(e) => {
                    error!("Failed to store Pet address: {}", e);
                }
            }
        }
        
        info!("Generated and stored {} Pet addresses in batch", generated_count);
    }
    
    pub async fn get_current_count(&self) -> Result<usize> {
        self.storage.count_addresses()
    }
}