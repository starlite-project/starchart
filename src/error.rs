//! The different errors within the crate.

use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
};

#[cfg(feature = "accessor")]
pub use crate::accessor::{AccessorError, AccessorErrorType};
#[doc(inline)]
#[cfg(feature = "action")]
pub use crate::action::{
	ActionError, ActionErrorType, ActionRunError, ActionRunErrorType, ActionValidationError,
	ActionValidationErrorType,
};
#[cfg(feature = "fs")]
#[doc(inline)]
pub use crate::backend::{FsError, FsErrorType};
#[cfg(feature = "memory")]
pub use crate::backend::{MemoryError, MemoryErrorType};

// NOTE: This error shouldn't be used anywhere inside this crate, it's only meant for end users as an ease of use
// error struct.

/// An error that occurred within the crate.
#[derive(Debug)]
pub struct Error {
	source: Option<Box<dyn StdError + Send + Sync>>,
	kind: ErrorType,
}

impl Error {
	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &ErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn StdError + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(self) -> (ErrorType, Option<Box<dyn StdError + Send + Sync>>) {
		(self.kind, self.source)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			#[cfg(feature = "memory")]
			ErrorType::Memory => f.write_str("an error occurred with the memory backend"),
			#[cfg(feature = "fs")]
			ErrorType::Fs => f.write_str("an error occurred with a file-system backend"),
			#[cfg(feature = "action")]
			ErrorType::ActionRun => f.write_str("an error occurred running an action"),
			#[cfg(feature = "action")]
			ErrorType::ActionValidation => f.write_str("an action is invalid"),
			#[cfg(feature = "accessor")]
			ErrorType::Accessor => f.write_str("an error occurred accessing data"),
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn StdError + 'static))
	}
}

#[cfg(feature = "memory")]
impl From<MemoryError> for Error {
	fn from(e: MemoryError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::Memory,
		}
	}
}

#[cfg(feature = "fs")]
impl From<FsError> for Error {
	fn from(e: FsError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::Fs,
		}
	}
}

#[cfg(feature = "action")]
impl From<ActionError> for Error {
	fn from(e: ActionError) -> Self {
		let kind = match e.kind() {
			ActionErrorType::Run => ErrorType::ActionRun,
			ActionErrorType::Validation => ErrorType::ActionValidation,
		};
		Self {
			// source will always be an ActionRunError or ActionValidationError
			source: e.into_source(),
			kind,
		}
	}
}

#[cfg(feature = "action")]
impl From<ActionValidationError> for Error {
	fn from(e: ActionValidationError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::ActionValidation,
		}
	}
}

#[cfg(feature = "action")]
impl From<ActionRunError> for Error {
	fn from(e: ActionRunError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::ActionRun,
		}
	}
}

#[cfg(feature = "accessor")]
impl From<AccessorError> for Error {
	fn from(e: AccessorError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::Accessor,
		}
	}
}

/// The type of [`Error`] that occurred.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[non_exhaustive]
pub enum ErrorType {
	/// An error occurred in the [`MemoryBackend`].
	///
	/// [`MemoryBackend`]: crate::backend::MemoryBackend
	#[cfg(feature = "memory")]
	Memory,
	/// An error occurred with a [`FsBackend`].
	///
	/// [`FsBackend`]: crate::backend::fs::FsBackend
	#[cfg(feature = "fs")]
	Fs,
	/// An [`ActionValidationError`] occurred.
	#[cfg(feature = "action")]
	ActionValidation,
	/// An [`ActionRunError`] occurred.
	#[cfg(feature = "action")]
	ActionRun,
	/// An [`AccessorError`] occurred.
	#[cfg(feature = "accessor")]
	Accessor,
}
