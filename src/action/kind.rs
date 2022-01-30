use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	str::FromStr,
};

use serde::{Deserialize, Serialize};

use super::{ActionValidationError, ActionValidationErrorType};

/// The type of [`CRUD`] action to perform
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
#[must_use = "getting the information on what action will be performed has no side effects"]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg(feature = "action")]
pub enum ActionKind {
	/// Signifies that the operation will be a Create.
	///
	/// This locks the database and allows no other reads or writes until it is complete.
	Create,
	/// Signifies that the operation will be a Read.
	///
	/// This allows multiple different readers, but doesn't allow writing until all Reads are complete.
	Read,
	/// Signifies that the operation will be an Update.
	///
	/// This locks the database and allows no other reads or writes until it is complete.
	Update,
	/// Signifies that the operation will be a Delete.
	///
	/// This locks the database and allows no other reads or writes until it is complete.
	Delete,
}

impl Display for ActionKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Create => f.write_str("Create"),
			Self::Read => f.write_str("Read"),
			Self::Update => f.write_str("Update"),
			Self::Delete => f.write_str("Delete"),
		}
	}
}

impl FromStr for ActionKind {
	type Err = ActionValidationError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Create" => Ok(Self::Create),
			"Read" => Ok(Self::Read),
			"Update" => Ok(Self::Update),
			"Delete" => Ok(Self::Delete),
			_ => Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Parse,
			}),
		}
	}
}

impl Default for ActionKind {
	fn default() -> Self {
		Self::Read
	}
}

#[cfg(test)]
mod tests {
	use std::fmt::{Debug, Display};

	use serde::{Deserialize, Serialize};
	use static_assertions::assert_impl_all;

	use super::ActionKind;

	assert_impl_all!(
		ActionKind: Clone,
		Copy,
		Debug,
		Default,
		Deserialize<'static>,
		Display,
		PartialEq,
		Send,
		Serialize,
		Sync
	);

	#[test]
	fn default() {
		assert_eq!(ActionKind::default(), ActionKind::Read);
	}

	#[test]
	fn display() {
		assert_eq!(ActionKind::Create.to_string(), "Create");
		assert_eq!(ActionKind::Read.to_string(), "Read");
		assert_eq!(ActionKind::Update.to_string(), "Update");
		assert_eq!(ActionKind::Delete.to_string(), "Delete");
	}
}
