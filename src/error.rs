//! The different errors within the crate.

use thiserror::Error;

#[cfg(feature = "json")]
#[doc(cfg(feature = "json"))]
pub use crate::backend::JsonError;

#[cfg(feature = "cache")]
#[doc(cfg(feature = "cache"))]
pub use crate::backend::CacheError;

pub use crate::database::DatabaseError;

/// An unknown error has occurred, this is an ease of use type and should not be relied on.
#[derive(Debug, Default, Error, Clone, Copy)]
#[error("an unknown error has occurred")]
pub struct UnknownError;

/// An error enum to wrap around all possible errors within the crate.
#[derive(Debug, Error)]
pub enum ChartError {
    /// A [`JsonError`] has occurred.
    #[cfg(feature = "json")]
    #[doc(cfg(feature = "json"))]
    #[error(transparent)]
    Json(#[from] JsonError),
    /// A [`CacheError`] has occurred.
    #[cfg(feature = "cache")]
    #[doc(cfg(feature = "cache"))]
    #[error(transparent)]
    Cache(#[from] CacheError),
    /// A [`DatabaseError`] has occurred.
    #[error(transparent)]
    Database(DatabaseError),
    /// A custom error has occurred, this is useful for [`Result`] return types.
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync>),
}
