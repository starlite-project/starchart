use thiserror::Error;

/// An error occurred during validation of an [`Action`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ActionValidationError {
	/// The [`OperationTarget`] was not set.
	#[error("an invalid operation was set")]
	InvalidOperation,
	/// No data was passed when data was expected.
	#[error("no data was given when data was expected")]
	NoData,
	/// No key was passed when a key was expected.
	#[error("no key was given when a key was expected.")]
	NoKey,
	/// Attempted to [`ActionKind::Update`] an [`OperationTarget::Table`].
	#[error("updating an entire table is unsupported")]
	UpdatingTable,
	/// No table was provided.
	#[error("no table was provided")]
	NoTable,
}

/// An error that occurred from running an [`Action`].
#[derive(Debug, Error)]
#[error("an error occurred running the action")]
pub struct ActionRunError {
	#[from]
	source: Box<dyn std::error::Error + Send + Sync>,
}
