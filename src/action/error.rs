use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActionError<E: Error + 'static> {
	#[error(transparent)]
	Run(#[from] ActionRunError<E>),
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
}

/// An error that occurred from running an [`Action`].
///
/// [`Action`]: crate::action::Action
#[derive(Debug, Error)]
#[error("an error occurred running the action")]
pub enum ActionRunError<E: Error> {
	Backend(#[from] E),
}
