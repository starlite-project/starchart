#![allow(missing_copy_implementations)]

//! The action structs for CRUD operations.

mod r#impl;
mod kind;
pub mod result;
mod target;

use std::{
	cell::Cell,
	error::Error,
	fmt::{Debug, Formatter, Result as FmtResult},
	future::Future,
	marker::PhantomData,
	pin::Pin,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[doc(inline)]
pub use self::{
	kind::ActionKind,
	r#impl::{
		ActionRunner, CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OpTarget,
		ReadOperation, TableTarget, UpdateOperation,
	},
	result::ActionResult,
	target::OperationTarget,
};
use crate::{backend::Backend, Entry, Gateway, IndexEntry, Key};

/// A type alias for an [`Action`] with [`CreateOperation`] and [`EntryTarget`] as the parameters.
pub type CreateEntryAction<S> = Action<S, CreateOperation, EntryTarget>;

impl<S: Entry + 'static, E: Error> ActionRunner<(), ActionError<E>>
	for Action<S, CreateOperation, EntryTarget>
{
	unsafe fn __run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionError<E>>> + Send>> {
		Box::pin(async move {
			// SAFETY: table_name is asserted to be true in `Action::validate`
			let table_name = self.inner.table_name.unwrap_unchecked();

			let entry = self.inner.data.unwrap_unchecked();

			todo!()
		})
	}

	unsafe fn __validate(&self) -> Result<(), ActionError> {
		self.validate()
	}
}

/// A type alias for an [`Action`] with [`ReadOperation`] and [`EntryTarget`] as the parameters.
pub type ReadEntryAction<S> = Action<S, ReadOperation, EntryTarget>;

impl<S: Entry, E: Error> ActionRunner<S, ActionError<E>> for Action<S, ReadOperation, EntryTarget> {
	unsafe fn __run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<S, ActionError<E>>> + Send>> {
		Box::pin(async move { todo!() })
	}

	unsafe fn __validate(&self) -> Result<(), ActionError> {
		self.validate()
	}
}

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`EntryTarget`] as the parameters.
pub type UpdateEntryAction<S> = Action<S, UpdateOperation, EntryTarget>;

impl<S: Entry, E: Error> ActionRunner<(), ActionError<E>>
	for Action<S, UpdateOperation, EntryTarget>
{
	unsafe fn __run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionError<E>>> + Send>> {
		Box::pin(async move { todo!() })
	}

	unsafe fn __validate(&self) -> Result<(), ActionError> {
		self.validate()
	}
}

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`EntryTarget`] as the parameters.
pub type DeleteEntryAction<S> = Action<S, DeleteOperation, EntryTarget>;

impl<S: Entry, E: Error> ActionRunner<bool, ActionError<E>>
	for Action<S, DeleteOperation, EntryTarget>
{
	unsafe fn __run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<bool, ActionError<E>>> + Send>> {
		Box::pin(async move { todo!() })
	}

	unsafe fn __validate(&self) -> Result<(), ActionError> {
		self.validate()
	}
}

/// A type alias for an [`Action`] with [`CreateOperation`] and [`TableTarget`] as the parameters.
pub type CreateTableAction<S> = Action<S, CreateOperation, TableTarget>;

impl<S: Entry, E: Error> ActionRunner<(), ActionError<E>>
	for Action<S, CreateOperation, TableTarget>
{
	unsafe fn __run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionError<E>>> + Send>> {
		Box::pin(async move { todo!() })
	}

	unsafe fn __validate(&self) -> Result<(), ActionError> {
		self.validate()
	}
}

/// A type alias for an [`Action`] with [`ReadOperation`] and [`TableTarget`] as the parameters.
pub type ReadTableAction<S> = Action<S, ReadOperation, TableTarget>;

// this is only here to satisfy the `clippy::type_complexity` lint
type ReadTableResult<S, E> = Pin<Box<dyn Future<Output = Result<Vec<S>, ActionError<E>>> + Send>>;

impl<S: Entry, E: Error> ActionRunner<Vec<S>, ActionError<E>>
	for Action<S, ReadOperation, TableTarget>
{
	unsafe fn __run<B: Backend>(self, gateway: &Gateway<B>) -> ReadTableResult<S, E> {
		Box::pin(async move { todo!() })
	}

	unsafe fn __validate(&self) -> Result<(), ActionError> {
		self.validate()
	}
}

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`TableTarget`] as the parameters.
pub type UpdateTableAction<S> = Action<S, UpdateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`TableTarget`] as the parameters.
pub type DeleteTableAction<S> = Action<S, DeleteOperation, TableTarget>;

impl<S: Entry, E: Error> ActionRunner<bool, ActionError<E>>
	for Action<S, DeleteOperation, TableTarget>
{
	unsafe fn __run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<bool, ActionError<E>>> + Send>> {
		Box::pin(async move { todo!() })
	}

	unsafe fn __validate(&self) -> Result<(), ActionError> {
		self.validate()
	}
}

/// An error occurred during validation of an [`Action`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ActionError<E: Error = !> {
	/// The [`OperationTarget`] was not set.
	#[error("an invalid operation was set")]
	InvalidOperation,
	/// No data was passed when data was expected.
	#[error("no data was given when data was expected")]
	NoData,
	/// No key was passed when a key was expected.
	#[error("no key was given when a key was expected.")]
	NoKey,
	/// Attempted to [`ActionKind::Update`] an [`OperationTarget::Table`].
	#[error("updating an entire table is unsupported")]
	UpdatingTable,
	/// An error occurred from the [`Backend`].
	///
	/// [`Backend`]: crate::backend::Backend
	#[error(transparent)]
	Backend(#[from] E),
	/// No table was provided.
	#[error("no table was provided")]
	NoTable,
}

/// An [`Action`] for easy [`CRUD`] operations within a [`Gateway`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Gateway`]: crate::Gateway
#[derive(Serialize, Deserialize)]
#[must_use = "an action alone has no side effects"]
pub struct Action<S, C: CrudOperation, T: OpTarget> {
	pub(crate) inner: InternalAction<S, C, T>,
	pub(crate) validated: Cell<bool>,
}

impl<S, C: CrudOperation, T: OpTarget> Action<S, C, T> {
	/// Creates a new [`Action`] with the specified operation.
	pub fn new() -> Self {
		Self {
			inner: InternalAction::new(),
			validated: Cell::new(false),
		}
	}

	/// Returns the [`ActionKind`] we will be performing with said action.
	pub fn kind(&self) -> ActionKind {
		self.inner.kind()
	}

	/// Returns the [`OperationTarget`] we will be performing with said action.
	#[must_use]
	pub fn target(&self) -> OperationTarget {
		self.inner.target()
	}

	/// Whether the [`Action`] has been validated.
	#[must_use]
	pub fn is_validated(&self) -> bool {
		self.validated.get()
	}
}

impl<S: Entry, T: OpTarget> Action<S, CreateOperation, T> {
	/// Begins a [`CreateOperation`] action.
	pub fn create() -> Self {
		Self::new()
	}
}

impl<S: Entry, T: OpTarget> Action<S, ReadOperation, T> {
	/// Begins a [`ReadOperation`] action.
	pub fn read() -> Self {
		Self::new()
	}
}

impl<S: Entry, T: OpTarget> Action<S, UpdateOperation, T> {
	/// Begins an [`UpdateOperation`] action.
	pub fn update() -> Self {
		Self::new()
	}
}

impl<S: Entry, T: OpTarget> Action<S, DeleteOperation, T> {
	/// Begins a [`DeleteOperation`] action.
	pub fn delete() -> Self {
		Self::new()
	}
}

impl<S: Entry, C: CrudOperation> Action<S, C, TableTarget> {
	/// Creates a new [`TableTarget`] based operation.
	pub fn table() -> Self {
		Self::new()
	}
}

impl<S: Entry, C: CrudOperation> Action<S, C, EntryTarget> {
	/// Creates a new [`EntryTarget`] based operation.
	pub fn entry() -> Self {
		Self::new()
	}
}

impl<S: Entry, C: CrudOperation, T: OpTarget> Action<S, C, T> {
	/// Changes the [`CrudOperation`] of this [`Action`].
	pub fn into_operation<O: CrudOperation>(self) -> Action<S, O, T> {
		Action {
			inner: self.inner.into_operation(),
			validated: self.validated,
		}
	}

	/// Changes the [`OpTarget`] of this [`Action`].
	pub fn into_target<T2: OpTarget>(self) -> Action<S, C, T2> {
		Action {
			inner: self.inner.into_target(),
			validated: self.validated,
		}
	}

	/// Sets the [`CrudOperation`] of this [`Action`] to [`CreateOperation`].
	pub fn into_create(self) -> Action<S, CreateOperation, T> {
		self.into_operation()
	}

	/// Sets the [`CrudOperation`] of this [`Action`] to [`ReadOperation`].
	pub fn into_read(self) -> Action<S, ReadOperation, T> {
		self.into_operation()
	}

	/// Sets the [`CrudOperation`] of this [`Action`] to [`UpdateOperation`].
	pub fn into_update(self) -> Action<S, UpdateOperation, T> {
		self.into_operation()
	}

	/// Sets the [`CrudOperation`] of this [`Action`] to [`DeleteOperation`].
	pub fn into_delete(self) -> Action<S, DeleteOperation, T> {
		self.into_operation()
	}

	/// Sets the [`OpTarget`] of this [`Action`] to [`TableTarget`].
	pub fn into_table(self) -> Action<S, C, TableTarget> {
		self.into_target()
	}

	/// Sets the [`OpTarget`] of this [`Action`] to [`EntryTarget`].
	pub fn into_entry(self) -> Action<S, C, EntryTarget> {
		self.into_target()
	}

	/// Sets the target [`Entry`] of this [`Action`].
	pub fn with_entry<S2>(self) -> Action<S2, C, T> {
		Action {
			inner: self.inner.with_entry(),
			validated: self.validated,
		}
	}

	/// Validates the [`Action`].
	///
	/// This is a no-op if the [`Action`] has already been validated.
	///
	/// # Errors
	///
	/// Returns an [`ActionError::InvalidOperation`] if the [`Action`] has not set an [`OperationTarget`].
	pub fn validate(&self) -> Result<(), ActionError> {
		if self.is_validated() {
			return Ok(());
		}

		if self.target() == OperationTarget::Unknown {
			return Err(ActionError::InvalidOperation);
		}

		if self.needs_data() && self.inner.data.is_none() {
			return Err(ActionError::NoData);
		}

		if self.needs_key() && self.inner.key.is_none() {
			return Err(ActionError::NoKey);
		}

		if self.is_updating_table() {
			return Err(ActionError::UpdatingTable);
		}

		if self.inner.key.is_none() {
			return Err(ActionError::NoTable);
		}

		self.validated.set(true);

		Ok(())
	}

	/// Sets the key for the action.
	///
	/// Users should prefer to call [`Self::set_entry`] over this, as setting the
	/// entry will automatically call this.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.inner.set_key(key.to_key());

		self.validated.set(false);

		self
	}

	/// Sets the data for the action.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_data(&mut self, entity: &S) -> &mut Self {
		self.inner.set_entry(Box::new(entity.clone()));

		self
	}

	// Updating tables is unsupported
	fn is_updating_table(&self) -> bool {
		self.kind() == ActionKind::Update && self.target() == OperationTarget::Table
	}

	fn needs_data(&self) -> bool {
		if self.kind() == ActionKind::Read {
			return false;
		}

		if self.kind() == ActionKind::Delete {
			return false;
		}

		if self.target() == OperationTarget::Table {
			return false;
		}

		true
	}

	fn needs_key(&self) -> bool {
		if self.target() == OperationTarget::Table {
			return false;
		}

		true
	}
}

impl<S: IndexEntry, C: CrudOperation, T: OpTarget> Action<S, C, T> {
	/// Sets the [`Entry`] and [`Key`] that this [`Action`] will act over.
	pub fn set_entry(&mut self, entity: &S) -> &mut Self {
		self.set_key(&entity.key());

		self.set_data(entity);

		self
	}
}

impl<S: Entry + Debug, C: CrudOperation, T: OpTarget> Debug for Action<S, C, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Action")
			.field("kind", &self.kind())
			.field("table_name", &self.inner.table_name)
			.field("data", &self.inner.data)
			.field("key", &self.inner.key)
			.field("target", &self.target())
			.finish()
	}
}

impl<S: Entry + Clone, C: CrudOperation, T: OpTarget> Clone for Action<S, C, T> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			validated: self.validated.clone(),
		}
	}
}

impl<S: Entry, C: CrudOperation, T: OpTarget> Default for Action<S, C, T> {
	fn default() -> Self {
		Self {
			inner: InternalAction::default(),
			validated: Cell::default(),
		}
	}
}

// This struct is used for database creation and interaction
// within the crate, and performs no validation
// to ensure optimizations, and SHOULD NOT be exposed to public API.
#[derive(Serialize, Deserialize)]
pub(crate) struct InternalAction<S, C: CrudOperation, T: OpTarget> {
	kind: PhantomData<C>,
	table_name: Option<String>,
	data: Option<Box<S>>,
	key: Option<String>,
	target: PhantomData<T>,
}

impl<S, C: CrudOperation, T: OpTarget> InternalAction<S, C, T> {
	pub(crate) fn new() -> Self {
		Self {
			kind: PhantomData,
			table_name: None,
			data: None,
			key: None,
			target: PhantomData,
		}
	}

	#[allow(clippy::unused_self)]
	pub(crate) fn kind(&self) -> ActionKind {
		C::kind()
	}

	#[allow(clippy::unused_self)]
	pub(crate) fn target(&self) -> OperationTarget {
		T::target()
	}

	pub(crate) fn into_target<New: OpTarget>(self) -> InternalAction<S, C, New> {
		InternalAction {
			kind: PhantomData,
			table_name: self.table_name,
			data: self.data,
			key: self.key,
			target: PhantomData,
		}
	}

	pub(crate) fn into_operation<New: CrudOperation>(self) -> InternalAction<S, New, T> {
		InternalAction {
			kind: PhantomData,
			table_name: self.table_name,
			data: self.data,
			key: self.key,
			target: PhantomData,
		}
	}

	pub(crate) fn with_entry<S2>(self) -> InternalAction<S2, C, T> {
		InternalAction {
			kind: self.kind,
			table_name: self.table_name,
			data: None,
			key: None,
			target: self.target,
		}
	}
}

impl<S: Entry, C: CrudOperation, O: OpTarget> InternalAction<S, C, O> {
	pub(crate) fn set_table_name(&mut self, table_name: String) -> &mut Self {
		self.table_name = Some(table_name);

		self
	}

	pub(crate) fn set_key(&mut self, key: String) -> &mut Self {
		self.key = Some(key);

		self
	}

	pub(crate) fn set_entry(&mut self, entity: Box<S>) -> &mut Self {
		self.data = Some(entity);

		self
	}

	pub(crate) fn set_data(&mut self, data: S) -> &mut Self {
		self.data = Some(Box::new(data));

		self
	}
}

impl<S: Entry + Clone, C: CrudOperation, T: OpTarget> Clone for InternalAction<S, C, T> {
	fn clone(&self) -> Self {
		Self {
			kind: PhantomData,
			table_name: self.table_name.clone(),
			data: self.data.clone(),
			key: self.key.clone(),
			target: PhantomData,
		}
	}
}

impl<S: Entry, C: CrudOperation, T: OpTarget> Default for InternalAction<S, C, T> {
	fn default() -> Self {
		Self {
			kind: PhantomData,
			table_name: Option::default(),
			data: Option::default(),
			key: Option::default(),
			target: PhantomData,
		}
	}
}
