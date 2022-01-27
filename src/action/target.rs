use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

/// The target of the [`CRUD`] operation.
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg(feature = "action")]
#[must_use = "action target information is useless if unused."]
pub enum TargetKind {
	/// The operation will be performed on a table.
	Table,
	/// The operation will be performed on a single entry.
	Entry,
}

impl Default for TargetKind {
	fn default() -> Self {
		Self::Entry
	}
}

impl Display for TargetKind {
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

	use super::TargetKind;

	assert_impl_all!(
		TargetKind: Clone,
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
		assert_eq!(TargetKind::default(), TargetKind::Entry);
	}

	#[test]
	fn display() {
		assert_eq!(TargetKind::Entry.to_string(), "Entry");
		assert_eq!(TargetKind::Table.to_string(), "Table");
	}
}
