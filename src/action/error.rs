use std::{error::Error, fmt::Debug};

use thiserror::Error;

use super::RESTRICTED_KEYS;

/// A general [`Action`] error.
///
/// [`Action`]: crate::action::Action
#[derive(Debug, Error)]
pub enum ActionError<E: Error + 'static> {
	/// An error occurred during [`Action::run`].
	///
	/// [`Action::run`]: crate::action::ActionRunner::run
	#[error(transparent)]
	Run(#[from] ActionRunError<E>),
	/// An error occurred during [`Action::validate`].
	///
	/// [`Action::validate`]: crate::action::ActionRunner::validate
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
	/// A provided key or table name was one of the restricted names.
	#[cfg(feature = "metadata")]
	#[cfg_attr(docsrs, doc(cfg(feature = "metadata")))]
	#[error("the `{}` keys are restricted", RESTRICTED_KEYS.as_ref().join(", "))]
	RestrictedKey,
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
	#[cfg_attr(docsrs, doc(cfg(feature = "metadata")))]
	#[error("invalid entry was provided, {0} does not match the metadata for table `{1}`")]
	Metadata(&'static str, String),
}
