use std::{error::Error, fmt::Debug};

use thiserror::Error;

/// A general [`Action`] error.
///
/// [`Action`]: crate::action::Action
#[derive(Debug, Error)]
pub enum ActionError<E: Error + 'static> {
	/// An error occurred during [`Action::run`].
	///
	/// [`Action::run`]: crate::Action::run
	#[error(transparent)]
	Run(#[from] ActionRunError<E>),
	/// An error occurred during any of the action validation methods.
	#[error(transparent)]
	Validation(#[from] ActionValidationError),
}

/// An error occurred during validation of an [`Action`].
///
/// [`Action`]: crate::action::Action
#[derive(Debug, Error, Clone, Copy)]
#[non_exhaustive]
pub enum ActionValidationError {
	/// No data was passed when data was expected.
	#[error("no data was given when data was expected")]
	Data,
	/// No key was passed when a key was expected.
	#[error("no key was given when a key was expected.")]
	Key,
	/// No table was provided.
	#[error("no table was provided")]
	Table,
	/// A provided key or table name was "metadata", which is restricted
	#[cfg(feature = "metadata")]
	#[error("the `__metadata__` key is restricted")]
	Metadata,
}

/// An error that occurred from running an [`Action`].
///
/// [`Action`]: crate::action::Action
#[derive(Debug, Error)]
pub enum ActionRunError<E: Error> {
	/// An error occurred from the [`Backend`].
	///
	/// [`Backend`]: crate::backend::Backend
	#[error(transparent)]
	Backend(#[from] E),
	/// An invalid [`Entry`] was provided.
	///
	/// [`Entry`]: crate::Entry
	#[cfg(feature = "metadata")]
	#[error("invalid entry was provided, {0} does not match the metadata for table `{1}`")]
	Metadata(&'static str, String),
}
