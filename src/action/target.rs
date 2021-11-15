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
