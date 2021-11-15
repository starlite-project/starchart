use thiserror::Error;

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
pub struct ActionRunError {
	#[from]
	source: Box<dyn std::error::Error + Send + Sync>,
}
