use crate::{backend::Backend, Database};
use dashmap::DashMap;
use futures::executor::block_on;
use std::sync::Arc;

/// todo
#[derive(Debug, Clone)]
pub struct Gateway<B: Backend> {
    backend: Arc<B>,
    databases: DashMap<String, Database<B>>,
}

impl<B: Backend> Gateway<B> {
    /// todo
    ///
    /// # Errors
    ///
    /// todo
    pub async fn new(backend: B) -> Result<Self, B::Error> {
        backend.init().await?;
        Ok(Self {
            backend: Arc::new(backend),
            databases: DashMap::new(),
        })
    }
}

impl<B: Backend> Drop for Gateway<B> {
    fn drop(&mut self) {
        block_on(unsafe { self.backend.shutdown() });
    }
}
