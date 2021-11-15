//! The different errors within the crate.

use thiserror::Error;

use crate::backend::Backend;
#[cfg(feature = "cache")]
#[doc(cfg(feature = "cache"))]
pub use crate::backend::CacheError;
#[cfg(feature = "json")]
#[doc(cfg(feature = "json"))]
pub use crate::backend::JsonError;
#[doc(inline)]
pub use crate::{
	action::{ActionRunError, ActionValidationError},
	database::DatabaseError,
};

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
	/// An [`ActionValidationError`] has occurred.
	#[error(transparent)]
	ActionValidation(#[from] ActionValidationError),
	/// An [`ActionRunError`] has occurred.
	#[error(transparent)]
	ActionRunError(#[from] ActionRunError),
	/// A custom error has occurred, this is useful for [`Result`] return types.
	#[error(transparent)]
	Custom(#[from] Box<dyn std::error::Error + Send + Sync>),
}
