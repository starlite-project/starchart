use std::{fmt::Debug, future::Future, pin::Pin};

use serde::{Deserialize, Serialize};

use super::{ActionKind, OperationTarget};
use crate::{backend::Backend, Gateway};

/// The marker trait for all action runs, this trait should not be used and is only used
/// to make the return type of [`Gateway::run`] easily known.
///
/// [`Gateway::run`]: crate::Gateway::run
pub trait ActionRunner<Success, Failure>: private::Sealed + Send {
	#[doc(hidden)]
	unsafe fn run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<Success, Failure>> + Send>>;

	// TODO: move `Action::validate` logic into this function.
	#[doc(hidden)]
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
