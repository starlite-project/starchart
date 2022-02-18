//! The different errors within the crate.

use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
};

#[doc(inline)]
pub use crate::action::{
	ActionError, ActionErrorType, ActionRunError, ActionRunErrorType, ActionValidationError,
	ActionValidationErrorType,
};

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

	/// Creates a new error from a backend.
	#[must_use]
	pub fn backend(e: Option<Box<dyn StdError + Send + Sync>>) -> Self {
		Self {
			source: e,
			kind: ErrorType::Backend,
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			ErrorType::Backend => f.write_str("an error occurred within a backend"),
			ErrorType::ActionRun => f.write_str("an error occurred running an action"),
			ErrorType::ActionValidation => f.write_str("an action is invalid"),
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

impl From<ActionValidationError> for Error {
	fn from(e: ActionValidationError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::ActionValidation,
		}
	}
}

impl From<ActionRunError> for Error {
	fn from(e: ActionRunError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ErrorType::ActionRun,
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
	/// An [`ActionValidationError`] occurred.
	ActionValidation,
	/// An [`ActionRunError`] occurred.
	ActionRun,
}
