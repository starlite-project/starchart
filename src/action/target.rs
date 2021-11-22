use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

/// The target of the [`CRUD`] operation.
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OperationTarget {
	/// The operation will be performed on a table.
	Table,
	/// The operation will be performed on a single entry.
	Entry,
}

impl Default for OperationTarget {
	fn default() -> Self {
		Self::Entry
	}
}

impl Display for OperationTarget {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Table => f.write_str("Table"),
			Self::Entry => f.write_str("Entry"),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::fmt::{Debug, Display};

	use serde::{Deserialize, Serialize};
	use static_assertions::assert_impl_all;

	use super::OperationTarget;

	assert_impl_all!(
		OperationTarget: Clone,
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
		assert_eq!(OperationTarget::default(), OperationTarget::Entry);
	}

	#[test]
	fn display() {
		assert_eq!(OperationTarget::Entry.to_string(), "Entry");
		assert_eq!(OperationTarget::Table.to_string(), "Table");
	}
}
