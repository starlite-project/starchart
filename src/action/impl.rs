use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::{ActionKind, TargetKind};

/// Marker type for a Create operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg(feature = "active")]
#[non_exhaustive]
pub struct CreateOperation;

/// Marker type for a Read operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg(feature = "active")]
#[non_exhaustive]
pub struct ReadOperation;

/// Marker type for an Update operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg(feature = "active")]
#[non_exhaustive]
pub struct UpdateOperation;

/// Marker type for a Delete operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg(feature = "active")]
#[non_exhaustive]
pub struct DeleteOperation;

/// A sealed marker trait for helping an [`Action`] represent what type of operation will occur.
///
/// [`Action`]: crate::action::Action
#[cfg(feature = "active")]
pub trait CrudOperation: private::Sealed {
	#[doc(hidden)]
	fn kind() -> ActionKind;
}

impl CrudOperation for CreateOperation {
	fn kind() -> ActionKind {
		ActionKind::Create
	}
}

impl CrudOperation for ReadOperation {
	fn kind() -> ActionKind {
		ActionKind::Read
	}
}

impl CrudOperation for UpdateOperation {
	fn kind() -> ActionKind {
		ActionKind::Update
	}
}

impl CrudOperation for DeleteOperation {
	fn kind() -> ActionKind {
		ActionKind::Delete
	}
}

/// Marker type for a table operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg(feature = "active")]
#[non_exhaustive]
pub struct TableTarget;

/// Marker type for an entry operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg(feature = "active")]
#[non_exhaustive]
pub struct EntryTarget;

/// A sealed marker trait for helping an [`Action`] represent what type of target the
/// operation will cover.
///
/// [`Action`]: crate::action::Action
#[cfg(feature = "active")]
pub trait OperationTarget: private::Sealed {
	#[doc(hidden)]
	fn target() -> TargetKind;
}

impl OperationTarget for TableTarget {
	fn target() -> TargetKind {
		TargetKind::Table
	}
}

impl OperationTarget for EntryTarget {
	fn target() -> TargetKind {
		TargetKind::Entry
	}
}

mod private {
	use super::{
		CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OperationTarget,
		ReadOperation, TableTarget, UpdateOperation,
	};
	use crate::{Action, Entry};

	pub trait Sealed {}

	impl Sealed for CreateOperation {}
	impl Sealed for ReadOperation {}
	impl Sealed for UpdateOperation {}
	impl Sealed for DeleteOperation {}
	impl Sealed for TableTarget {}
	impl Sealed for EntryTarget {}
	impl<S: Entry, C: CrudOperation, T: OperationTarget> Sealed for Action<S, C, T> {}
}

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use serde::{Deserialize, Serialize};
	use static_assertions::assert_impl_all;

	use super::{
		CreateOperation, DeleteOperation, EntryTarget, ReadOperation, TableTarget, UpdateOperation,
	};
	use crate::action::{ActionKind, CrudOperation, OperationTarget, TargetKind};

	assert_impl_all!(
		CreateOperation: Clone,
		Copy,
		Debug,
		Deserialize<'static>,
		Send,
		Serialize,
		Sync
	);
	assert_impl_all!(
		ReadOperation: Clone,
		Copy,
		Debug,
		Deserialize<'static>,
		Send,
		Serialize,
		Sync
	);
	assert_impl_all!(
		UpdateOperation: Clone,
		Copy,
		Debug,
		Deserialize<'static>,
		Send,
		Serialize,
		Sync
	);
	assert_impl_all!(
		DeleteOperation: Clone,
		Copy,
		Debug,
		Deserialize<'static>,
		Send,
		Serialize,
		Sync
	);
	assert_impl_all!(
		TableTarget: Clone,
		Copy,
		Debug,
		Deserialize<'static>,
		Send,
		Serialize,
		Sync
	);
	assert_impl_all!(
		EntryTarget: Clone,
		Copy,
		Debug,
		Deserialize<'static>,
		Send,
		Serialize,
		Sync
	);

	#[test]
	fn kind() {
		assert_eq!(CreateOperation::kind(), ActionKind::Create);
		assert_eq!(ReadOperation::kind(), ActionKind::Read);
		assert_eq!(UpdateOperation::kind(), ActionKind::Update);
		assert_eq!(DeleteOperation::kind(), ActionKind::Delete);
	}

	#[test]
	fn target() {
		assert_eq!(TableTarget::target(), TargetKind::Table);
		assert_eq!(EntryTarget::target(), TargetKind::Entry);
	}
}
