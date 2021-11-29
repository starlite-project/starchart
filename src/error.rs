//! The different errors within the crate.

use thiserror::Error;

use crate::backend::Backend;
#[cfg(feature = "cache")]
#[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
pub use crate::backend::CacheError;
#[cfg(feature = "fs")]
#[cfg_attr(docsrs, doc(cfg(feature = "fs")))]
pub use crate::backend::FsError;
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub use crate::backend::JsonError;
#[doc(inline)]
pub use crate::{
	action::{ActionError, ActionRunError, ActionValidationError},
	database::DatabaseError,
};

// NOTE: This error shouldn't be used anywhere inside this crate, it's only meant for end users as an ease of use
// error struct.
// It would also cause Generic Hell.

/// An error enum to wrap around all possible errors within the crate.
#[derive(Debug, Error)]
pub enum ChartError<B: Backend> {
	/// A [`CacheError`] has occurred.
	#[cfg(feature = "cache")]
	#[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
	#[error(transparent)]
	Cache(#[from] CacheError),
	/// A [`FsError`] has occurred.
	#[cfg(feature = "fs")]
	#[cfg_attr(docsrs, doc(cfg(feature = "fs")))]
	#[error(transparent)]
	Fs(#[from] FsError),
	/// A [`DatabaseError`] has occurred.
	#[error(transparent)]
	Database(#[from] DatabaseError<B::Error>),
	/// An [`ActionValidationError`] has occurred.
	#[error(transparent)]
	ActionValidation(#[from] ActionValidationError),
	/// An [`ActionRunError`] has occurred.
	#[error(transparent)]
	ActionRunError(#[from] ActionRunError<B::Error>),
	/// A custom error has occurred, this is useful for [`Result`] return types.
	#[error(transparent)]
	Custom(#[from] Box<dyn std::error::Error + Send + Sync>),
}
