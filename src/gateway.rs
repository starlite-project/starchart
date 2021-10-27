use crate::{backend::Backend, Database};
use futures::executor::block_on;
use std::{collections::HashMap, sync::Arc};

/// todo
#[derive(Debug, Clone)]
pub struct Gateway<B: Backend> {
    backend: Arc<B>,
    databases: HashMap<String, Database<B>>,
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
            databases: HashMap::new(),
        })
    }
}

impl<B: Backend> Drop for Gateway<B> {
    fn drop(&mut self) {
        let fut = self.backend.shutdown();

        block_on(fut).expect("failed to shutdown (this is really bad)");
    }
}
