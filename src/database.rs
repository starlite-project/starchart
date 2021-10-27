#![allow(dead_code)]
use crate::backend::Backend;
use std::sync::Arc;

/// todo
#[derive(Debug, Clone)]
pub struct Database<B: Backend> {
    table_name: String,
    backend: Arc<B>,
}

impl<B: Backend> Database<B> {
    /// todo
    pub(crate) fn new(table_name: String, backend: Arc<B>) -> Self {
        Self {
            table_name,
            backend,
        }
    }
}
