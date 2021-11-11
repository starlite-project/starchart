//! The different errors within the crate.

use crate::backend::Backend;
use thiserror::Error;

#[cfg(feature = "json")]
#[doc(cfg(feature = "json"))]
pub use crate::backend::JsonError;

#[cfg(feature = "cache")]
#[doc(cfg(feature = "cache"))]
pub use crate::backend::CacheError;

#[doc(inline)]
pub use crate::{action::ActionError, database::DatabaseError};

// NOTE: This error shouldn't be used anywhere inside this crate, it's only meant for end users as an ease of use
// error struct.
// It would also cause Generic Hell.

/// An error enum to wrap around all possible errors within the crate.
#[derive(Debug, Error)]
pub enum ChartError<B: Backend> {
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
	Database(#[from] DatabaseError<B::Error>),
	/// An [`ActionError`] has occurred.
	#[error(transparent)]
	Action(#[from] ActionError),
	/// A custom error has occurred, this is useful for [`Result`] return types.
	#[error(transparent)]
	Custom(#[from] Box<dyn std::error::Error + Send + Sync>),
}
