//! The different errors within the crate.

use thiserror::Error;

#[doc(inline)]
pub use crate::action::{ActionError, ActionRunError, ActionValidationError};
use crate::backend::Backend;
#[cfg(feature = "fs")]
pub use crate::backend::FsError;
#[cfg(feature = "memory")]
pub use crate::backend::MemoryError;

// NOTE: This error shouldn't be used anywhere inside this crate, it's only meant for end users as an ease of use
// error struct.
// It would also cause Generic Hell.

/// An error enum to wrap around all possible errors within the crate.
#[derive(Debug, Error)]
pub enum ChartError<B: Backend> {
	/// A [`MemoryError`] has occurred.
	#[cfg(feature = "memory")]
	#[error(transparent)]
	Memory(#[from] MemoryError),
	/// A [`FsError`] has occurred.
	#[cfg(feature = "fs")]
	#[error(transparent)]
	Fs(#[from] FsError),
	/// An [`ActionValidationError`] has occurred.
	#[error(transparent)]
	ActionValidation(#[from] ActionValidationError),
	/// An [`ActionRunError`] has occurred.
	#[error(transparent)]
	ActionRun(#[from] ActionRunError<B::Error>),
	/// A custom error has occurred, this is useful for [`Result`] return types.
	#[error(transparent)]
	Custom(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl<B: Backend> From<ActionError<<B as Backend>::Error>> for ChartError<B> {
	fn from(e: ActionError<<B as Backend>::Error>) -> Self {
		match e {
			ActionError::Run(e) => Self::ActionRun(e),
			ActionError::Validation(e) => Self::ActionValidation(e),
		}
	}
}
