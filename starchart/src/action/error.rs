use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
};

use crate::backend::Backend;

#[derive(Debug)]
pub struct ActionError {
	pub(super) source: Option<Box<dyn StdError + Send + Sync>>,
	pub(super) kind: ActionErrorType,
}

impl ActionError {
	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &ActionErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn StdError + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(self) -> (ActionErrorType, Option<Box<dyn StdError + Send + Sync>>) {
		(self.kind, self.source)
	}

	pub(super) fn from_backend<E: StdError + Send + Sync + 'static>(e: E) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: ActionErrorType::Backend,
		}
	}
}

impl Display for ActionError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			ActionErrorType::SomethingMissing(missing) => match missing {
				MissingValue::Data => f.write_str("no data was given when data was expected"),
				MissingValue::Key => f.write_str("no key was given when a key was expected"),
				MissingValue::Table => {
					f.write_str("an operation was attempted on a non-existent table")
				}
			},
			#[cfg(feature = "metadata")]
			ActionErrorType::Metadata(value) => {
				if let Some(table) = value {
					f.write_str("metadata mismatch for table ")?;
					Display::fmt(&table, f)
				} else {
					f.write_str("the `__metadata__` key is restricted")
				}
			}
			ActionErrorType::Backend => f.write_str("an error occurred within the backend"),
		}
	}
}

impl StdError for ActionError {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn StdError + 'static))
	}
}

#[derive(Debug)]
pub enum ActionErrorType {
	SomethingMissing(MissingValue),
	#[cfg(feature = "metadata")]
	Metadata(Option<String>),

	Backend,
}

#[derive(Debug)]
pub enum MissingValue {
	Data,
	Key,
	Table,
}
