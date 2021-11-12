//! Represents the many different results from actions.

#![allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]

use std::{convert::TryFrom, ops::Deref};

use thiserror::Error;

use super::{ActionKind, OperationTarget};
use crate::Entry;

/// Trait for all the variants of [`ActionResult`] to easily convert
/// between a table and entity [`Result`].
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait MultiResult: private::Sealed
where
	Self: Sized,
{
	/// The result type for a table operation.
	type TableResult;

	/// The result type for an entity operation.
	type EntityResult;

	/// Similar to [`Result::ok`], returning the [`Self::TableResult`] if there was one and [`None`] if not.
	fn table(self) -> Option<Self::TableResult>;

	/// Similar to [`Result::ok`], returning the [`Self::EntityResult`] if there was one and [`None`] if not.
	fn entity(self) -> Option<Self::EntityResult>;

	/// Similar to [`Result::unwrap`], returning the [`Self::TableResult`], panicking otherwise.
	#[track_caller]
	fn unwrap_table(self) -> Self::TableResult {
		self.table()
			.expect("called `MultiResult::unwrap_table()` on a `Entity` value")
	}

	/// Similar to [`Result::unwrap`], returning the [`Self::EntityResult`], panicking otherwise.
	#[track_caller]
	fn unwrap_entity(self) -> Self::EntityResult {
		self.entity()
			.expect("called `MultiResult::unwrap_entity()` on a `Table` value")
	}

	/// Similar to [`Result::unwrap_unchecked`], returning the [`Self::TableResult`] without checking if
	/// it's valid first.
	///
	/// # Safety
	///
	/// Calling this method on a [`Self::EntityResult`] is *[undefined behavior]*.
	///
	/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
	unsafe fn unwrap_table_unchecked(self) -> Self::TableResult {
		self.table().unwrap_unchecked()
	}

	/// Similar to [`Result::unwrap_unchecked`], returning the [`Self::EntityResult`] without checking if
	/// it's valid first.
	///
	/// # Safety
	///
	/// Calling this method on a [`Self::TableResult`] is *[undefined behavior]*.
	///
	/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
	unsafe fn unwrap_entity_unchecked(self) -> Self::EntityResult {
		self.entity().unwrap_unchecked()
	}
}

/// The base [`Result`] type for [`Action`]s.
///
/// [`Action`]: crate::action::Action
#[derive(Debug)]
#[must_use = "this `ActionResult` may be an Error of some kind, which should be handled"]
pub enum ActionResult<T: Entry> {
	/// The result from an [`Action::create`].
	///
	/// [`Action::create`]: crate::action::Action::create
	Create(CreateResult),
	/// The result from an [`Action::read`].
	///
	/// [`Action::read`]: crate::action::Action::read
	Read(ReadResult<T>),
	/// The result from an [`Action::update`].
	///
	/// [`Action::update`]: crate::action::Action::update
	Update(UpdateResult),
	/// The result from an [`Action::delete`].
	///
	/// [`Action::delete`]: crate::action::Action::delete
	Delete(DeleteResult),
}

impl<T: Entry> ActionResult<T> {
	/// Converts from [`ActionResult`] to [`Option`].
	///
	/// This consumes `self`, returning a [`CreateResult`] if the [`ActionResult`] is a [`CreateResult`],
	/// and [`None`] otherwise.
	pub fn create(self) -> Option<CreateResult> {
		match self {
			Self::Create(result) => Some(result),
			_ => None,
		}
	}

	/// Converts from [`ActionResult`] to [`Option`].
	///
	/// This consumes `self`, returning a [`ReadResult`] if the [`ActionResult`] is a [`ReadResult`],
	/// and [`None`] otherwise.
	pub fn read(self) -> Option<ReadResult<T>> {
		match self {
			Self::Read(result) => Some(result),
			_ => None,
		}
	}

	/// Converts from [`ActionResult`] to [`Option`].
	///
	/// This consumes `self`, returning an [`UpdateResult`] if the [`ActionResult`] is an [`UpdateResult`],
	/// and [`None`] otherwise.
	pub fn update(self) -> Option<UpdateResult> {
		match self {
			Self::Update(result) => Some(result),
			_ => None,
		}
	}

	/// Converts from [`ActionResult`] to [`Option`].
	///
	/// This consumes `self`, returning an [`DeleteResult`] if the [`ActionResult`] is an [`DeleteResult`],
	/// and [`None`] otherwise.
	pub fn delete(self) -> Option<DeleteResult> {
		match self {
			Self::Delete(result) => Some(result),
			_ => None,
		}
	}

	/// Returns the [`ActionKind`] that this [`ActionResult`] represents.
	pub fn kind(&self) -> ActionKind {
		match self {
			Self::Create(_) => ActionKind::Create,
			Self::Read(_) => ActionKind::Read,
			Self::Update(_) => ActionKind::Update,
			Self::Delete(_) => ActionKind::Delete,
		}
	}

	/// Returns the [`CreateResult`] from this [`ActionResult`].
	///
	/// # Panics
	///
	/// Panics if the [`ActionResult`] is not a [`CreateResult`].
	#[track_caller]
	pub fn unwrap_create(self) -> CreateResult {
		let kind = self.kind();

		self.create()
			.unwrap_or_else(|| panic!("called `ActionResult::unwrap_create` on a `{}` value", kind))
	}

	/// Returns the [`ReadResult`] from this [`ActionResult`].
	///
	/// # Panics
	///
	/// Panics if the [`ActionResult`] is not a [`ReadResult`].
	#[track_caller]
	pub fn unwrap_read(self) -> ReadResult<T> {
		let kind = self.kind();

		self.read()
			.unwrap_or_else(|| panic!("called `ActionResult::unwrap_read` on a `{}` value", kind))
	}

	/// Returns the [`UpdateResult`] from this [`ActionResult`].
	///
	/// # Panics
	///
	/// Panics if the [`ActionResult`] is not a [`UpdateResult`].
	#[track_caller]
	pub fn unwrap_update(self) -> UpdateResult {
		let kind = self.kind();

		self.update()
			.unwrap_or_else(|| panic!("called `ActionResult::unwrap_update` on a `{}` value", kind))
	}

	/// Returns the [`DeleteResult`] from this [`ActionResult`].
	///
	/// # Panics
	///
	/// Panics if the [`ActionResult`] is not a [`DeleteResult`].
	#[track_caller]
	pub fn unwrap_delete(self) -> DeleteResult {
		let kind = self.kind();

		self.delete()
			.unwrap_or_else(|| panic!("called `ActionResult::unwrap_delete` on a `{}` value", kind))
	}
}

/// A result from an [`Action::create`].
///
/// [`Action::create`]: crate::action::Action::create
#[derive(Debug)]
#[must_use = "this `CreateResult` may be an Error of some kind, which should be handled"]
pub enum CreateResult {
	/// A table creation result.
	Table(Result<(), CreateError>),
	/// An entity creation result.
	Entity(Result<(), CreateError>),
}

impl MultiResult for CreateResult {
	type EntityResult = Result<(), CreateError>;
	type TableResult = Result<(), CreateError>;

	fn table(self) -> Option<Self::TableResult> {
		if let Self::Table(r) = self {
			Some(r)
		} else {
			None
		}
	}

	fn entity(self) -> Option<Self::EntityResult> {
		if let Self::Entity(r) = self {
			Some(r)
		} else {
			None
		}
	}
}

impl From<CreateResult> for Result<(), CreateError> {
	fn from(res: CreateResult) -> Self {
		match res {
			CreateResult::Entity(r) | CreateResult::Table(r) => r,
		}
	}
}

impl Deref for CreateResult {
	type Target = Result<(), CreateError>;

	fn deref(&self) -> &Self::Target {
		match self {
			CreateResult::Table(r) | CreateResult::Entity(r) => r,
		}
	}
}

/// An error occurred during an [`Action::create`].
///
/// [`Action::create`]: crate::action::Action::create
#[derive(Debug, Error)]
#[error("an error happened during {target} creation")]
pub struct CreateError {
	source: Box<dyn std::error::Error>,
	target: OperationTarget,
}

impl CreateError {
	/// The target the create operation was for.
	pub const fn target(&self) -> OperationTarget {
		self.target
	}
}

/// A result from an [`Action::read`].
///
/// [`Action::read`]: crate::action::Action::read
#[derive(Debug)]
#[must_use = "this `ReadResult` may be an Error of some kind, which should be handled"]
pub enum ReadResult<T: Entry> {
	/// A table read result.
	Table(Result<Vec<T>, ReadError>),
	/// An entry read result.
	///
	/// # Note
	///
	/// The return result will be a [`Vec`] with just one element, so to get the value indexing by 0 will
	/// never fail.
	///
	/// However, if one wishes to get the inner value without indexing, the [`MultiResult`] impl
	/// provides easy to use methods to get said values.
	Entity(Result<Vec<T>, ReadError>),
}

impl<T: Entry> MultiResult for ReadResult<T> {
	type EntityResult = Result<T, ReadError>;
	type TableResult = Result<Vec<T>, ReadError>;

	fn table(self) -> Option<Self::TableResult> {
		if let Self::Table(r) = self {
			Some(r)
		} else {
			None
		}
	}

	fn entity(self) -> Option<Self::EntityResult> {
		if let Self::Entity(r) = self {
			Some(r.map(|val| val[0].clone()))
		} else {
			None
		}
	}
}

impl<T: Entry> Deref for ReadResult<T> {
	type Target = Result<Vec<T>, ReadError>;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Table(r) | Self::Entity(r) => r,
		}
	}
}

/// Represents a conversion error when using the [`TryFrom`] impls for [`ReadResult`].
#[derive(Debug, Error, Clone, Copy)]
pub enum InvalidTargetError {
	/// Attempted to convert a [`ReadResult::Table`] into a [`Result<T, ReadError>`].
	#[error("attempted conversion of entity result into table")]
	ExpectedTable,
	/// Attempted to convert a [`ReadResult::Entity`] into a [`Result<Vec<T>, ReadError>`].
	#[error("attempted conversion of table result into entity")]
	ExpectedEntity,
}

impl<T: Entry> TryFrom<ReadResult<T>> for Result<Vec<T>, ReadError> {
	type Error = InvalidTargetError;

	fn try_from(value: ReadResult<T>) -> Result<Self, Self::Error> {
		if let ReadResult::Table(r) = value {
			Ok(r)
		} else {
			Err(InvalidTargetError::ExpectedTable)
		}
	}
}

impl<T: Entry> TryFrom<ReadResult<T>> for Result<T, ReadError> {
	type Error = InvalidTargetError;

	fn try_from(value: ReadResult<T>) -> Result<Self, Self::Error> {
		if let ReadResult::Entity(r) = value {
			Ok(r.map(|v| v[0].clone()))
		} else {
			Err(InvalidTargetError::ExpectedEntity)
		}
	}
}

/// An error occurred during an [`Action::read`].
///
/// [`Action::read`]: crate::action::Action::read
#[derive(Debug, Error)]
#[error("an error happened during {target} read")]
pub struct ReadError {
	source: Box<dyn std::error::Error>,
	target: OperationTarget,
}

impl ReadError {
	/// The target the read operation was for.
	pub const fn target(&self) -> OperationTarget {
		self.target
	}
}

/// A result from an [`Action::update`].
///
/// [`Action::update`]: crate::action::Action::update
#[derive(Debug)]
#[must_use = "this `UpdateResult` may be an Error of some kind, which should be handled"]
pub enum UpdateResult {
	/// A table update result.
	Table(Result<(), UpdateError>),
	/// An entity update result.
	Entity(Result<(), UpdateError>),
}

impl MultiResult for UpdateResult {
	type EntityResult = Result<(), UpdateError>;
	type TableResult = Result<(), UpdateError>;

	fn table(self) -> Option<Self::TableResult> {
		if let Self::Table(r) = self {
			Some(r)
		} else {
			None
		}
	}

	fn entity(self) -> Option<Self::EntityResult> {
		if let Self::Entity(r) = self {
			Some(r)
		} else {
			None
		}
	}
}

impl From<UpdateResult> for Result<(), UpdateError> {
	fn from(val: UpdateResult) -> Self {
		match val {
			UpdateResult::Entity(r) | UpdateResult::Table(r) => r,
		}
	}
}

impl Deref for UpdateResult {
	type Target = Result<(), UpdateError>;

	fn deref(&self) -> &Self::Target {
		match self {
			UpdateResult::Table(r) | UpdateResult::Entity(r) => r,
		}
	}
}

/// An error occurred during an [`Action::update`].
///
/// [`Action::update`]: crate::action::Action::update
#[derive(Debug, Error)]
#[error("an error happened during {target} update")]
pub struct UpdateError {
	source: Box<dyn std::error::Error>,
	target: OperationTarget,
}

impl UpdateError {
	/// The target the update operation was for.
	pub const fn target(&self) -> OperationTarget {
		self.target
	}
}

/// A result from an [`Action::delete`].
///
/// [`Action::delete`]: crate::action::Action::delete
#[derive(Debug)]
#[must_use = "this `DeleteResult` may be an Error of some kind, which should be handled"]
pub enum DeleteResult {
	/// A table delete result.
	Table(Result<bool, DeleteError>),
	/// An entity delete result.
	Entity(Result<bool, DeleteError>),
}

impl MultiResult for DeleteResult {
	type EntityResult = Result<bool, DeleteError>;
	type TableResult = Result<bool, DeleteError>;

	fn table(self) -> Option<Self::TableResult> {
		if let Self::Table(r) = self {
			Some(r)
		} else {
			None
		}
	}

	fn entity(self) -> Option<Self::EntityResult> {
		if let Self::Entity(r) = self {
			Some(r)
		} else {
			None
		}
	}
}

impl From<DeleteResult> for Result<bool, DeleteError> {
	fn from(value: DeleteResult) -> Self {
		match value {
			DeleteResult::Entity(r) | DeleteResult::Table(r) => r,
		}
	}
}

impl Deref for DeleteResult {
	type Target = Result<bool, DeleteError>;

	fn deref(&self) -> &Self::Target {
		match self {
			DeleteResult::Table(r) | DeleteResult::Entity(r) => r,
		}
	}
}

/// An error occurred during an [`Action::delete`].
///
/// [`Action::delete`]: crate::action::Action::delete
#[derive(Debug, Error)]
#[error("an error happened during {target} deletion")]
pub struct DeleteError {
	source: Box<dyn std::error::Error>,
	target: OperationTarget,
}

impl DeleteError {
	/// The target the delete operation was for.
	pub const fn target(&self) -> OperationTarget {
		self.target
	}
}

mod private {
	use super::{CreateResult, DeleteResult, ReadResult, UpdateResult};
	use crate::Entry;

	pub trait Sealed {}

	impl Sealed for CreateResult {}
	impl<T: Entry> Sealed for ReadResult<T> {}
	impl Sealed for UpdateResult {}
	impl Sealed for DeleteResult {}
}
