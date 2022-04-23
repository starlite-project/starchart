//! The different errors within the crate.

use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
};

#[doc(inline)]
pub use crate::action::{ActionError, ActionErrorType, MissingValue};

// NOTE: This error shouldn't be used anywhere inside this crate, it's only meant for end users as an ease of use
// error struct.

/// An error that occurred within the crate.
#[derive(Debug)]
pub struct Error {
	/// Optional source of the error.
	source: Option<Box<dyn StdError + Send + Sync>>,
	/// Type of error that occurred.
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

	/// Creates a new error from a backend.
	#[must_use]
	pub fn from_backend(e: Box<dyn StdError + Send + Sync>) -> Self {
		Self {
			source: Some(e),
			kind: ErrorType::Backend,
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			ErrorType::Backend => f.write_str("an error occurred within a backend"),
			ErrorType::Action => f.write_str("an error occurred with an action"),
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

impl From<ActionError> for Error {
	fn from(e: ActionError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::Action,
		}
	}
}

/// The type of [`Error`] that occurred.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[non_exhaustive]
pub enum ErrorType {
	/// An error occurred within a backend.
	Backend,
	/// An [`ActionError`] occurred.
	Action,
}
