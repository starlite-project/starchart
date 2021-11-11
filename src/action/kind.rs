use serde::{Deserialize, Serialize};

/// The type of [`CRUD`] action to perform
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
#[must_use = "getting the information on what action will be performed has no side effects"]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl Default for ActionKind {
	fn default() -> Self {
		Self::Read
	}
}
