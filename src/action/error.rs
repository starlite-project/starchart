use std::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
};

/// A general [`Action`] error.
///
/// [`Action`]: super::Action
#[derive(Debug)]
#[cfg(feature = "action")]
pub struct ActionError {
	source: Option<Box<dyn Error + Send + Sync>>,
	kind: ActionErrorType,
}

impl ActionError {
	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &ActionErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(self) -> (ActionErrorType, Option<Box<dyn Error + Send + Sync>>) {
		(self.kind, self.source)
	}
}

impl Display for ActionError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			ActionErrorType::Run => f.write_str("a run error occurred"),
			ActionErrorType::Validation => f.write_str("a validation error occurred"),
		}
	}
}

impl Error for ActionError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn Error + 'static))
	}
}

impl From<ActionRunError> for ActionError {
	fn from(err: ActionRunError) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: ActionErrorType::Run,
		}
	}
}

impl From<ActionValidationError> for ActionError {
	fn from(err: ActionValidationError) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: ActionErrorType::Validation,
		}
	}
}

/// The type of [`ActionError`] that occurred.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[non_exhaustive]
#[cfg(feature = "action")]
pub enum ActionErrorType {
	/// todo
	Run,
	/// A validation error has occurred.
	Validation,
}

/// An error occurred during validation of an [`Action`].
///
/// [`Action`]: super::Action
#[derive(Debug)]
#[cfg(feature = "action")]
pub struct ActionValidationError {
	pub(super) source: Option<Box<dyn Error + Send + Sync>>,
	pub(super) kind: ActionValidationErrorType,
}

impl ActionValidationError {
	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &ActionValidationErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(
		self,
	) -> (
		ActionValidationErrorType,
		Option<Box<dyn Error + Send + Sync>>,
	) {
		(self.kind, self.source)
	}
}

impl Display for ActionValidationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			ActionValidationErrorType::Data => {
				f.write_str("no data was given when data was expected")
			}
			ActionValidationErrorType::Key => {
				f.write_str("no key was given when a key was expected")
			}
			ActionValidationErrorType::Table => f.write_str("no table was provided"),
			#[cfg(feature = "metadata")]
			ActionValidationErrorType::Metadata => f.write_str("the `__metadata__` key is restricted"),
			ActionValidationErrorType::Conversion => f.write_str("an error occurred converting between dynamic and static actions"),
			ActionValidationErrorType::Parse => f.write_str("couldn't parse action data"),
		}
	}
}

impl Error for ActionValidationError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn Error + 'static))
	}
}

/// The type of [`ActionValidationError`] that occurred.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[cfg(feature = "action")]
#[non_exhaustive]
pub enum ActionValidationErrorType {
	/// Data was missing when it was expected.
	Data,
	/// A key was missing when it was expected.
	Key,
	/// The table was missing.
	Table,
	/// The table or key name matched the private metadata key.
	#[cfg(feature = "metadata")]
	Metadata,
	/// An invalid generic was passed during conversion.
	Conversion,
	/// The provided string was invalid.
	Parse,
}

/// An error that occurred from running an [`Action`].
///
/// [`Action`]: crate::action::Action
#[derive(Debug)]
#[cfg(feature = "action")]
pub struct ActionRunError {
	pub(super) source: Option<Box<dyn Error + Send + Sync>>,
	pub(super) kind: ActionRunErrorType,
}

impl ActionRunError {
	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &ActionRunErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(self) -> (ActionRunErrorType, Option<Box<dyn Error + Send + Sync>>) {
		(self.kind, self.source)
	}
}

impl Display for ActionRunError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			ActionRunErrorType::Backend => f.write_str("an error occurred within the backend"),
			#[cfg(feature = "metadata")]
			ActionRunErrorType::Metadata {
				type_name,
				table_name,
			} => {
				f.write_str("invalid entry was provided, ")?;
				Display::fmt(type_name, f)?;
				f.write_str("does not match the metadata for table ")?;
				Display::fmt(&table_name, f)
			}
		}
	}
}

impl Error for ActionRunError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn Error + 'static))
	}
}

/// The type of [`ActionRunError`] that occurred.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[non_exhaustive]
#[cfg(feature = "action")]
pub enum ActionRunErrorType {
	/// An error occurred with a [`Backend`] method.
	///
	/// [`Backend`]: crate::backend::Backend
	Backend,
	/// A value did not match the table's metadata.
	#[cfg(feature = "metadata")]
	Metadata {
		/// The name of the type that didn't match.
		type_name: &'static str,
		/// The table metadata to match against.
		table_name: String,
	},
}
