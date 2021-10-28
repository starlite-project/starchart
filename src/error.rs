//! todo

use thiserror::Error;

#[cfg(feature = "json")]
#[doc(cfg(feature = "json"))]
pub use crate::backend::JsonError;

#[cfg(feature = "cache")]
#[doc(cfg(feature = "cache"))]
pub use crate::backend::CacheError;

/// todo
#[derive(Debug, Error)]
pub enum ChartError {
    /// todo
    #[cfg(feature = "json")]
    #[doc(cfg(feature = "json"))]
    #[error(transparent)]
    Json(#[from] JsonError),
    /// todo
    #[cfg(feature = "cache")]
    #[doc(cfg(feature = "cache"))]
    #[error(transparent)]
    Cache(#[from] CacheError),
    /// todo
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync>),
}
