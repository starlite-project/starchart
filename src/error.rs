//! todo

use thiserror::Error;

#[cfg(feature = "json")]
#[doc(cfg(feature = "json"))]
pub use crate::backend::JsonError;

/// todo
#[derive(Debug, Error)]
pub enum ChartError {
    /// todo
    #[cfg(feature = "json")]
    #[doc(cfg(feature = "json"))]
    #[error(transparent)]
    Json(#[from] JsonError),
    /// todo
    #[error("an unknown error has occurred {0}")]
    Custom(&'static str)
}