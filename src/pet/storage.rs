use anyhow::{Result, Context};
use crossbeam_queue::SegQueue;
use sled::Db;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tokio::sync::RwLock;

use super::address::{PetAddress, PetAddressInfo};

/// High-performance storage with zero-copy lock-free queue for API hot path
/// Architecture:
/// - Hot path (API): Lock-free SegQueue for O(1) pop operations
/// - Cold path (backup): Sled DB for persistence and recovery
/// - Background: Async batch flush to avoid blocking
#[derive(Clone)]
pub struct PetStorage {
    // Hot path: Lock-free queue for instant API access
    address_queue: Arc<SegQueue<PetAddressInfo>>,

    // Metrics: Lock-free atomic counters
    queue_size: Arc<AtomicUsize>,
    counter: Arc<AtomicU64>,

    // Cold path: Persistence (optional, for backup only)
    db: Option<Arc<RwLock<Db>>>,
}

impl PetStorage {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db = sled::open(db_path)?;

        // Load existing counter from DB
        let counter = db.get(b"counter")?
            .map(|bytes| {
                let mut array = [0u8; 8];
                array.copy_from_slice(&bytes);
                u64::from_be_bytes(array)
            })
            .unwrap_or(0);

        // Restore addresses from DB to queue (during initialization, synchronous is fine)
        let address_queue = Arc::new(SegQueue::new());
        let mut count = 0;

        for result in db.scan_prefix(b"address:") {
            let (_key, value) = result?;
            let address_info: PetAddressInfo = serde_json::from_slice(&value)
                .context("Failed to deserialize address info")?;

            address_queue.push(address_info);
            count += 1;
        }

        tracing::info!("Restored {} addresses from database to queue", count);

        let storage = Self {
            address_queue,
            queue_size: Arc::new(AtomicUsize::new(count)),
            counter: Arc::new(AtomicU64::new(counter)),
            db: Some(Arc::new(RwLock::new(db))),
        };

        Ok(storage)
    }

    /// Store address - uses lock-free queue, no blocking
    pub fn store_address(&self, address: PetAddress) -> Result<u64> {
        let id = self.next_id();
        let address_info = PetAddressInfo {
            id,
            address,
            created_at: chrono::Utc::now(),
        };

        // Push to lock-free queue - O(1), non-blocking
        self.address_queue.push(address_info.clone());
        self.queue_size.fetch_add(1, Ordering::Relaxed);

        // Async persist to DB (fire-and-forget, no blocking)
        if let Some(db) = &self.db {
            let db = Arc::clone(db);
            let info = address_info.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::persist_address_async(db, info).await {
                    tracing::warn!("Background persistence failed: {}", e);
                }
            });
        }

        Ok(id)
    }

    /// Get next address - lock-free pop, zero blocking, O(1)
    pub fn get_next_address(&self) -> Result<Option<PetAddressInfo>> {
        match self.address_queue.pop() {
            Some(address_info) => {
                self.queue_size.fetch_sub(1, Ordering::Relaxed);

                // Async remove from DB (fire-and-forget)
                if let Some(db) = &self.db {
                    let db = Arc::clone(db);
                    let id = address_info.id;
                    tokio::spawn(async move {
                        if let Err(e) = Self::remove_address_async(db, id).await {
                            tracing::warn!("Background removal failed: {}", e);
                        }
                    });
                }

                Ok(Some(address_info))
            }
            None => Ok(None),
        }
    }

    /// Count addresses - O(1) atomic read, zero blocking
    pub fn count_addresses(&self) -> Result<usize> {
        Ok(self.queue_size.load(Ordering::Relaxed))
    }

    /// Clear all addresses - fast queue drain
    pub fn clear_all_addresses(&self) -> Result<()> {
        while self.address_queue.pop().is_some() {
            self.queue_size.fetch_sub(1, Ordering::Relaxed);
        }

        // Clear DB in background
        if let Some(db) = &self.db {
            let db = Arc::clone(db);
            tokio::spawn(async move {
                if let Err(e) = Self::clear_db_async(db).await {
                    tracing::warn!("Background clear failed: {}", e);
                }
            });
        }

        Ok(())
    }

    /// Get next ID - lock-free atomic increment
    fn next_id(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }

    /// Async persist to DB (non-blocking background operation)
    async fn persist_address_async(db: Arc<RwLock<Db>>, address_info: PetAddressInfo) -> Result<()> {
        let key = format!("address:{:010}", address_info.id);
        let value = serde_json::to_vec(&address_info)
            .context("Failed to serialize address info")?;

        let db = db.write().await;
        db.insert(key.as_bytes(), value)?;
        // Note: Removed flush() - sled auto-flushes periodically, no blocking needed

        Ok(())
    }

    /// Async remove from DB (non-blocking background operation)
    async fn remove_address_async(db: Arc<RwLock<Db>>, id: u64) -> Result<()> {
        let key = format!("address:{:010}", id);
        let db = db.write().await;
        db.remove(key.as_bytes())?;

        Ok(())
    }

    /// Async clear DB (non-blocking background operation)
    async fn clear_db_async(db: Arc<RwLock<Db>>) -> Result<()> {
        let db = db.write().await;
        let keys: Vec<_> = db.scan_prefix(b"address:")
            .map(|result| result.unwrap().0)
            .collect();

        for key in keys {
            db.remove(&key)?;
        }

        Ok(())
    }

    /// Start background task to periodically persist counter (every 10 seconds)
    pub fn start_counter_persistence(&self) {
        if let Some(db) = &self.db {
            let db = Arc::clone(db);
            let counter = Arc::clone(&self.counter);

            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

                    let current_counter = counter.load(Ordering::Relaxed);
                    let db = db.write().await;

                    if let Err(e) = db.insert(b"counter", &current_counter.to_be_bytes()) {
                        tracing::warn!("Failed to persist counter: {}", e);
                    } else {
                        tracing::debug!("Counter persisted: {}", current_counter);
                    }
                }
            });
        }
    }
}