use axum::{
    extract::{Request, ConnectInfo},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::sleep;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<DashMap<String, Vec<Instant>>>,
    max_requests: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        let limiter = Self {
            requests: Arc::new(DashMap::new()),
            max_requests,
            window_duration: Duration::from_secs(window_seconds),
        };
        
        // Start cleanup task
        let limiter_clone = limiter.clone();
        tokio::spawn(async move {
            limiter_clone.cleanup_task().await;
        });
        
        limiter
    }
    
    pub fn check_rate_limit(&self, ip: &str) -> bool {
        let now = Instant::now();
        let cutoff = now - self.window_duration;
        
        let mut entry = self.requests.entry(ip.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests
        entry.retain(|&time| time > cutoff);
        
        // Check if under limit
        if entry.len() >= self.max_requests as usize {
            false
        } else {
            // Add current request
            entry.push(now);
            true
        }
    }
    
    async fn cleanup_task(&self) {
        loop {
            sleep(Duration::from_secs(60)).await; // Cleanup every minute
            
            let now = Instant::now();
            let cutoff = now - self.window_duration;
            
            self.requests.retain(|_ip, times| {
                times.retain(|&time| time > cutoff);
                !times.is_empty()
            });
        }
    }
}

pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let limiter = request
        .extensions()
        .get::<RateLimiter>()
        .expect("RateLimiter not found in request extensions");
    
    let ip = addr.ip().to_string();
    
    if !limiter.check_rate_limit(&ip) {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded. Maximum 10 requests per minute allowed.",
        ).into_response();
    }
    
    next.run(request).await
}