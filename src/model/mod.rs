//! Model Layer

mod error;

pub use self::error::{Error, Result};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct ModelManager {
    pub pings_count: Arc<RwLock<HashMap<IpAddr, usize>>>,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        Ok(ModelManager {
            pings_count: Arc::default(),
        })
    }

    pub async fn add_request(&self, ip: IpAddr) -> Result<String> {
        let mut store = self.pings_count.write().await;

        let current = store.entry(ip).or_insert(0);
        *current += 1;

        Ok(format!(
            "You ip: {}, count request: {}",
            ip,
            store.get(&ip).unwrap()
        ))
    }
}
