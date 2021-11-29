use std::{fmt::Debug, future::Future, pin::Pin};

use serde::{Deserialize, Serialize};

use super::{ActionKind, OperationTarget};
use crate::{backend::Backend, Starchart};

/// The marker trait for all action runs, this trait should not be used and is only used
/// to make the return type of [`Starchart::run`] easily known.
///
/// See [`Actions trait-implementations`] for more information.
///
/// This trait is sealed and cannot be implemented outside of this crate.
///
/// [`Starchart::run`]: crate::Starchart::run
/// [`Actions trait-implementations`]: crate::action::Action#trait-implementations
pub trait ActionRunner<B: Backend, Success, Failure>: private::Sealed + Send {
	/// Runs the action through the [`Starchart`].
	///
	/// # Safety
	///
	/// This method may call a number of unsafe methods, such as [`Result::unwrap_unchecked`] and [`Option::unwrap_unchecked`].
	///
	/// However, the [`Action`] is guaranteed to be safe to run if [`ActionRunner::validate`] is called beforehand, as
	/// any issues found will be reported before.
	///
	/// [`Action`]: crate::action::Action
	unsafe fn run<'a>(
		self,
		gateway: &'a Starchart<B>,
	) -> Pin<Box<dyn Future<Output = Result<Success, Failure>> + Send + 'a>>;
	/// Validates that the [`Action`] has been created correctly.
	///
	/// Each individual implementation of this is specialized, for example,
	/// creating a table doesn't have to check for a valid key to have been set.
	///
	/// If calling [`Self::run`] manually, this should be called first to avoid any unwanted behavior when performing
	/// database operations.
	///
	/// [`Action`]: crate::action::Action
	///
	/// # Errors
	///
	/// Any type of [`ActionValidationError`] that can arise.
	///
	/// [`ActionValidationError`]: crate::action::ActionValidationError
	fn validate(&self) -> Result<(), super::ActionValidationError>;
}

/// Marker type for a Create operation.
#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct CreateOperation;

/// Marker type for a Read operation.
#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct ReadOperation;

/// Marker type for an Update operation.
#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct UpdateOperation;

/// Marker type for a Delete operation.
#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct DeleteOperation;

/// A sealed marker trait for helping an [`Action`] represent what type of operation will occur.
///
/// [`Action`]: crate::action::Action
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
#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct TableTarget;

/// Marker type for an entry operation.
#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct EntryTarget;

/// A sealed marker trait for helping an [`Action`] represent what type of target the
/// operation will cover.
///
/// [`Action`]: crate::action::Action
pub trait OpTarget: private::Sealed {
	#[doc(hidden)]
	fn target() -> OperationTarget;
}

impl OpTarget for TableTarget {
	fn target() -> OperationTarget {
		OperationTarget::Table
	}
}

impl OpTarget for EntryTarget {
	fn target() -> OperationTarget {
		OperationTarget::Entry
	}
}

mod private {
	use super::{
		CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OpTarget, ReadOperation,
		TableTarget, UpdateOperation,
	};
	use crate::{Action, Entry};

	pub trait Sealed {}

	impl Sealed for CreateOperation {}
	impl Sealed for ReadOperation {}
	impl Sealed for UpdateOperation {}
	impl Sealed for DeleteOperation {}
	impl Sealed for TableTarget {}
	impl Sealed for EntryTarget {}
	impl<S: Entry, C: CrudOperation, T: OpTarget> Sealed for Action<S, C, T> {}
}

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use serde::{Deserialize, Serialize};
	use static_assertions::assert_impl_all;

	use super::{
		CreateOperation, DeleteOperation, EntryTarget, ReadOperation, TableTarget, UpdateOperation,
	};
	use crate::action::{ActionKind, CrudOperation, OpTarget, OperationTarget};

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
		assert_eq!(TableTarget::target(), OperationTarget::Table);
		assert_eq!(EntryTarget::target(), OperationTarget::Entry);
	}
}
