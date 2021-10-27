#![allow(dead_code)]
use crate::backend::Backend;

/// todo
#[derive(Debug, Clone, Copy)]
pub struct Database<B: Backend> {
    backend: B,
}

impl<B: Backend> Database<B> {
    /// todo
    pub fn new(backend: B) -> Self {
        Self {
            backend,
        }
    }
}