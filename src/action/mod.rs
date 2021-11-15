#![allow(missing_copy_implementations)]

//! The action structs for CRUD operations.

// TODO: move error types to their own module and clean up all the impl blocks

mod error;
mod r#impl;
mod kind;
pub mod result;
mod target;

use std::{
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
	error::{ActionRunError, ActionValidationError},
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

impl<S: Entry + 'static> ActionRunner<(), ActionRunError>
	for Action<S, CreateOperation, EntryTarget>
{
	unsafe fn run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionRunError>> + Send>> {
		Box::pin(async move {
			let table_name = self.inner.table_name.unwrap();

			let entry = self.inner.data.unwrap();

			todo!()
		})
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_key()?;

		Ok(())
	}
}

/// A type alias for an [`Action`] with [`ReadOperation`] and [`EntryTarget`] as the parameters.
pub type ReadEntryAction<S> = Action<S, ReadOperation, EntryTarget>;

impl<S: Entry> ActionRunner<S, ActionRunError> for Action<S, ReadOperation, EntryTarget> {
	unsafe fn run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<S, ActionRunError>> + Send>> {
		Box::pin(async move { todo!() })
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		todo!()
	}
}

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`EntryTarget`] as the parameters.
pub type UpdateEntryAction<S> = Action<S, UpdateOperation, EntryTarget>;

impl<S: Entry> ActionRunner<(), ActionRunError> for Action<S, UpdateOperation, EntryTarget> {
	unsafe fn run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionRunError>> + Send>> {
		Box::pin(async move { todo!() })
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		todo!()
	}
}

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`EntryTarget`] as the parameters.
pub type DeleteEntryAction<S> = Action<S, DeleteOperation, EntryTarget>;

impl<S: Entry> ActionRunner<bool, ActionRunError> for Action<S, DeleteOperation, EntryTarget> {
	unsafe fn run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<bool, ActionRunError>> + Send>> {
		Box::pin(async move { todo!() })
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		todo!()
	}
}

/// A type alias for an [`Action`] with [`CreateOperation`] and [`TableTarget`] as the parameters.
pub type CreateTableAction<S> = Action<S, CreateOperation, TableTarget>;

impl<S: Entry> ActionRunner<(), ActionRunError> for Action<S, CreateOperation, TableTarget> {
	unsafe fn run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionRunError>> + Send>> {
		Box::pin(async move { todo!() })
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		todo!()
	}
}

/// A type alias for an [`Action`] with [`ReadOperation`] and [`TableTarget`] as the parameters.
pub type ReadTableAction<S> = Action<S, ReadOperation, TableTarget>;

// this is only here to satisfy the `clippy::type_complexity` lint
type ReadTableResult<S> = Pin<Box<dyn Future<Output = Result<Vec<S>, ActionRunError>> + Send>>;

impl<S: Entry> ActionRunner<Vec<S>, ActionRunError> for Action<S, ReadOperation, TableTarget> {
	unsafe fn run<B: Backend>(self, gateway: &Gateway<B>) -> ReadTableResult<S> {
		Box::pin(async move { todo!() })
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		todo!()
	}
}

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`TableTarget`] as the parameters.
pub type UpdateTableAction<S> = Action<S, UpdateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`TableTarget`] as the parameters.
pub type DeleteTableAction<S> = Action<S, DeleteOperation, TableTarget>;

impl<S: Entry> ActionRunner<bool, ActionRunError> for Action<S, DeleteOperation, TableTarget> {
	unsafe fn run<B: Backend>(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<bool, ActionRunError>> + Send>> {
		Box::pin(async move { todo!() })
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		todo!()
	}
}

/// An [`Action`] for easy [`CRUD`] operations within a [`Gateway`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Gateway`]: crate::Gateway
#[derive(Serialize, Deserialize)]
#[must_use = "an action alone has no side effects"]
pub struct Action<S, C: CrudOperation, T: OpTarget> {
	pub(crate) inner: InternalAction<S, C, T>,
}

impl<S, C: CrudOperation, T: OpTarget> Action<S, C, T> {
	/// Creates a new [`Action`] with the specified operation.
	pub fn new() -> Self {
		Self {
			inner: InternalAction::new(),
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

	fn validate_key(&self) -> Result<(), ActionValidationError> {
		if self.inner.key.is_none() {
			return Err(ActionValidationError::NoKey);
		}

		Ok(())
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
		}
	}

	/// Changes the [`OpTarget`] of this [`Action`].
	pub fn into_target<T2: OpTarget>(self) -> Action<S, C, T2> {
		Action {
			inner: self.inner.into_target(),
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
		}
	}

	/// Sets the key for the action.
	///
	/// Users should prefer to call [`Self::set_entry`] over this, as setting the
	/// entry will automatically call this.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.inner.set_key(key.to_key());

		self
	}

	/// Sets the data for the action.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_data(&mut self, entity: &S) -> &mut Self {
		self.inner.set_entry(Box::new(entity.clone()));

		self
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
		}
	}
}

impl<S: Entry, C: CrudOperation, T: OpTarget> Default for Action<S, C, T> {
	fn default() -> Self {
		Self {
			inner: InternalAction::default(),
		}
	}
}

// This struct is used for database creation and interaction
// within the crate, and performs no validation
// to ensure optimizations, and SHOULD NOT be exposed to public API.

// TODO: merge this into Action
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
