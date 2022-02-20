//! The action structs for CRUD operations.

// TODO: Add overwrite option.

mod dynamic;
mod error;
mod r#impl;
mod kind;
mod result;
mod target;

#[cfg(feature = "metadata")]
use std::any::type_name;
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	iter::FromIterator,
	marker::PhantomData,
};

#[cfg(not(feature = "metadata"))]
use futures_util::future::ok;
use futures_util::Future;

#[doc(hidden)]
pub use self::error::{
	ActionError, ActionErrorType, ActionRunError, ActionRunErrorType, ActionValidationError,
	ActionValidationErrorType,
};
pub use self::{
	dynamic::DynamicAction,
	kind::ActionKind,
	r#impl::{
		CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OperationTarget,
		ReadOperation, TableTarget, UpdateOperation,
	},
	result::ActionResult,
	target::TargetKind,
};
#[cfg(feature = "metadata")]
use crate::METADATA_KEY;
use crate::{
	backend::Backend,
	util::{is_metadata, InnerUnwrap},
	Entry, IndexEntry, Key, Starchart,
};

/// A type alias for an [`Action`] with [`CreateOperation`] and [`EntryTarget`] as the parameters.
pub type CreateEntryAction<'a, S> = Action<'a, S, CreateOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`ReadOperation`] and [`EntryTarget`] as the parameters.
pub type ReadEntryAction<'a, S> = Action<'a, S, ReadOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`EntryTarget`] as the parameters.
pub type UpdateEntryAction<'a, S> = Action<'a, S, UpdateOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`EntryTarget`] as the parameters.
pub type DeleteEntryAction<'a, S> = Action<'a, S, DeleteOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`CreateOperation`] and [`TableTarget`] as the parameters.
pub type CreateTableAction<'a, S> = Action<'a, S, CreateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`ReadOperation`] and [`TableTarget`] as the parameters.
pub type ReadTableAction<'a, S> = Action<'a, S, ReadOperation, TableTarget>;

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`TableTarget`] as the parameters.
///
/// This action can never been ran.
pub type UpdateTableAction<'a, S> = Action<'a, S, UpdateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`TableTarget`] as the parameters.
pub type DeleteTableAction<'a, S> = Action<'a, S, DeleteOperation, TableTarget>;

#[derive(Debug)]
pub(crate) struct InnerAction<'a, S: ?Sized> {
	pub data: Option<&'a S>,
	pub key: Option<String>,
	pub table: Option<&'a str>,
}

impl<'a, S: ?Sized> InnerAction<'a, S> {
	const fn new() -> Self {
		Self {
			data: None,
			key: None,
			table: None,
		}
	}

	fn validate_entry(&self) -> Result<(), ActionValidationError> {
		self.validate_key()?;
		self.validate_data()
	}

	fn validate_table(&self) -> Result<(), ActionValidationError> {
		if self.table.is_none() {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Table,
			});
		}

		self.validate_metadata(self.table)
	}

	fn validate_data(&self) -> Result<(), ActionValidationError> {
		if self.data.is_none() {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Data,
			});
		}

		Ok(())
	}

	fn validate_key(&self) -> Result<(), ActionValidationError> {
		if self.key.is_none() {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Key,
			});
		}

		self.validate_metadata(self.key.as_deref())
	}

	#[cfg(feature = "metadata")]
	#[allow(clippy::unused_self)]
	fn validate_metadata(&self, key: Option<&str>) -> Result<(), ActionValidationError> {
		if key == Some(METADATA_KEY) {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Metadata,
			});
		}

		Ok(())
	}

	#[cfg(not(feature = "metadata"))]
	#[allow(clippy::unused_self)]
	fn validate_metadata(&self, _: Option<&str>) -> Result<(), ActionValidationError> {
		Ok(())
	}
}

impl<'a, S: Entry + ?Sized> InnerAction<'a, S> {
	#[cfg(feature = "metadata")]
	async fn check_metadata<B: Backend>(
		&self,
		backend: &B,
		table_name: &str,
	) -> Result<(), ActionRunError> {
		backend
			.get::<S>(table_name, METADATA_KEY)
			.await
			.map(|_| {})
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Metadata {
					type_name: type_name::<S>(),
					table_name: table_name.to_owned(),
				},
			})
	}

	#[cfg(not(feature = "metadata"))]
	fn check_metadata<B: Backend>(
		&self,
		_: &B,
		_: &str,
	) -> impl Future<Output = Result<(), ActionRunError>> {
		ok(())
	}

	async fn check_table<B: Backend>(
		&self,
		backend: &B,
		table: &str,
	) -> Result<(), ActionRunError> {
		if backend.has_table(table).await.map_err(|e| ActionRunError {
			source: Some(Box::new(e)),
			kind: ActionRunErrorType::Backend,
		})? {
			Ok(())
		} else {
			Err(ActionRunError {
				source: None,
				kind: ActionRunErrorType::MissingTable,
			})
		}
	}

	async fn create_entry<B: Backend>(mut self, chart: &Starchart<B>) -> Result<(), ActionError> {
		self.validate_entry()?;
		self.validate_table()?;

		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let (table, key, entry) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
				self.data.take().inner_unwrap(),
			)
		};

		self.check_table(backend, table).await?;
		self.check_metadata(backend, table).await?;

		backend
			.ensure(table, &key, &*entry)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		drop(lock);
		Ok(())
	}

	async fn read_entry<B: Backend>(
		mut self,
		chart: &Starchart<B>,
	) -> Result<Option<S>, ActionError> {
		self.validate_table()?;
		self.validate_key()?;

		let lock = chart.guard.shared().await;

		let backend = &**chart;

		let (table, key) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
			)
		};

		self.check_table(backend, table).await?;
		self.check_metadata(backend, table).await?;

		let res = backend.get(table, &key).await.map_err(|e| ActionRunError {
			source: Some(Box::new(e)),
			kind: ActionRunErrorType::Backend,
		})?;

		drop(lock);

		Ok(res)
	}

	async fn update_entry<B: Backend>(mut self, chart: &Starchart<B>) -> Result<(), ActionError> {
		self.validate_table()?;
		self.validate_entry()?;

		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let (table, key, entry) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
				self.data.take().inner_unwrap(),
			)
		};

		self.check_table(backend, table).await?;
		self.check_metadata(backend, table).await?;

		backend
			.update(table, &key, &*entry)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		drop(lock);

		Ok(())
	}

	async fn delete_entry<B: Backend>(mut self, chart: &Starchart<B>) -> Result<bool, ActionError> {
		self.validate_table()?;
		self.validate_key()?;
		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let (table, key) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
			)
		};

		self.check_table(backend, table).await?;
		self.check_metadata(backend, table).await?;

		if !backend.has(table, &key).await.map_err(|e| ActionRunError {
			source: Some(Box::new(e)),
			kind: ActionRunErrorType::Backend,
		})? {
			drop(lock);
			return Ok(false);
		}

		backend
			.delete(table, &key)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		drop(lock);

		Ok(true)
	}

	async fn create_table<B: Backend>(self, chart: &Starchart<B>) -> Result<(), ActionError> {
		self.validate_table()?;

		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let table = unsafe { self.table.inner_unwrap() };

		backend
			.ensure_table(table)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		#[cfg(feature = "metadata")]
		{
			let metadata = S::default();
			backend
				.ensure(table, METADATA_KEY, &metadata)
				.await
				.map_err(|e| ActionRunError {
					source: Some(Box::new(e)),
					kind: ActionRunErrorType::Metadata {
						type_name: type_name::<S>(),
						table_name: table.to_owned(),
					},
				})?;
		}

		drop(lock);

		Ok(())
	}

	async fn read_table<B: Backend, I>(mut self, chart: &Starchart<B>) -> Result<I, ActionError>
	where
		I: FromIterator<S>,
	{
		self.validate_table()?;
		let lock = chart.guard.shared().await;

		let backend = &**chart;

		let table = unsafe { self.table.take().inner_unwrap() };

		self.check_table(backend, table).await?;
		self.check_metadata(backend, table).await?;

		let keys = backend
			.get_keys::<Vec<_>>(table)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		let keys = keys
			.iter()
			.filter_map(|v| {
				if is_metadata(v) {
					None
				} else {
					Some(v.as_str())
				}
			})
			.collect::<Vec<_>>();

		let data = backend
			.get_all::<S, I>(table, &keys)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		drop(lock);

		Ok(data)
	}

	async fn delete_table<B: Backend>(mut self, chart: &Starchart<B>) -> Result<bool, ActionError> {
		self.validate_table()?;

		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let table = unsafe { self.table.take().inner_unwrap() };

		self.check_table(backend, table).await?;
		self.check_metadata(backend, table).await?;

		if !backend.has_table(table).await.map_err(|e| ActionRunError {
			source: Some(Box::new(e)),
			kind: ActionRunErrorType::Backend,
		})? {
			drop(lock);
			return Ok(false);
		}

		backend
			.delete_table(table)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		drop(lock);

		Ok(true)
	}
}

impl<'a, S: ?Sized> Default for InnerAction<'a, S> {
	fn default() -> Self {
		Self::new()
	}
}

impl<'a, S: ?Sized> Clone for InnerAction<'a, S> {
	fn clone(&self) -> Self {
		Self {
			key: self.key.clone(),
			data: self.data,
			table: self.table,
		}
	}
}

/// An [`Action`] for easy [`CRUD`] operations within a [`Starchart`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Starchart`]: crate::Starchart
#[derive(Clone)]
#[must_use = "an action alone has no side effects"]
#[repr(transparent)]
pub struct Action<'a, S, C, T> {
	pub(crate) inner: InnerAction<'a, S>,
	kind: PhantomData<C>,
	target: PhantomData<T>,
}

impl<'a, S, C, T> Action<'a, S, C, T> {
	/// Creates a new [`Action`] with the specified operation.
	pub const fn new() -> Self {
		Self {
			inner: InnerAction::new(),
			kind: PhantomData,
			target: PhantomData,
		}
	}

	/// Get a reference to the currently set table.
	#[must_use]
	pub const fn table(&self) -> Option<&str> {
		self.inner.table
	}

	/// Get a reference to the currently set key.
	#[must_use]
	pub fn key(&self) -> Option<&str> {
		self.inner.key.as_deref()
	}
}

impl<'a, S: Entry, C: CrudOperation, T: OperationTarget> Action<'a, S, C, T> {
	/// Construct a new [`Action`] with the specified table.
	pub fn with_table(table: &'a str) -> Self {
		let mut act = Self::new();

		act.set_table(table);

		act
	}

	/// Get a reference to the currently set data.
	#[must_use]
	pub fn data(&self) -> Option<&S> {
		self.inner.data
	}

	/// Returns the [`ActionKind`] we will be performing with said action.
	#[allow(clippy::unused_self)]
	pub fn kind(&self) -> ActionKind {
		C::kind()
	}

	/// Returns the [`OperationTarget`] we will be performing with said action.
	#[allow(clippy::unused_self)]
	pub fn target(&self) -> TargetKind {
		T::target()
	}

	/// Converts the action to a dynamic one, usually for serialization of some sort.
	pub fn to_dynamic(&self) -> DynamicAction<S> {
		DynamicAction {
			key: self.key().map(ToOwned::to_owned),
			table: self.table().map(ToOwned::to_owned),
			data: self.data().cloned().map(Box::new),
			kind: C::kind(),
			target: T::target(),
		}
	}

	/// Sets the table for this action.
	pub fn set_table(&mut self, table_name: &'a str) -> &mut Self {
		self.inner.table.replace(table_name);

		self // coverage:ignore-line
	}

	/// Validates that the table key is set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_table`] has not yet been called.
	pub fn validate_table(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_table()
	}

	/// Validates that the key is not the private metadata key.
	///
	/// # Errors
	/// Errors if [`Self::set_key`] was passed the private metadata key.
	#[cfg(feature = "metadata")]
	#[allow(clippy::unused_self)]
	pub fn validate_metadata(&self, key: Option<&str>) -> Result<(), ActionValidationError> {
		self.inner.validate_metadata(key)
	}

	/// Validates that the key is not the private metadata key.
	///
	/// # Errors
	/// Errors if [`Self::set_key`] was passed the private metadata key.
	#[cfg(not(feature = "metadata"))]
	#[allow(clippy::unused_self)]
	pub fn validate_metadata(&self, _: Option<&str>) -> Result<(), ActionValidationError> {
		Ok(())
	}
}

// Entry helpers
impl<'a, S: Entry, C: CrudOperation> Action<'a, S, C, EntryTarget> {
	/// Construct a new [`Action`] with the specified table and key.
	pub fn with_key<K: Key>(table: &'a str, key: &K) -> Self {
		let mut act = Self::new();

		act.set_table(table).set_key(key);
		act
	}

	/// Sets the key for the action.
	///
	/// Users should prefer to call [`Self::set_entry`] over this, as setting the
	/// entry will automatically call this.
	///
	/// This is unused on [`TargetKind::Table`] actions.
	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.inner.key.replace(key.to_key());

		self // coverage:ignore-line
	}

	/// Sets the data for the action.
	///
	/// This is unused on [`TargetKind::Table`] actions.
	pub fn set_data(&mut self, entity: &'a S) -> &mut Self {
		self.inner.data.replace(entity);

		self // coverage:ignore-line
	}

	/// Validate that the key has been set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_key`] has not yet been called.
	pub fn validate_key(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_key()
	}

	/// Validates that the data has been set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_data`] has not yet been called.
	pub fn validate_data(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_data()
	}

	/// Validates that both the key and data have been set.
	///
	/// # Errors
	///
	/// This errors if both the [`Self::set_key`] and [`Self::set_data`] (or [`Self::set_entry`]) has not been called.
	pub fn validate_entry(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_entry()
	}
}

impl<'a, S: IndexEntry, C: CrudOperation> Action<'a, S, C, EntryTarget> {
	/// Sets the [`Entry`] and [`Key`] that this [`Action`] will act over.
	pub fn set_entry(&mut self, entity: &'a S) -> &mut Self {
		self.set_key(entity.key()).set_data(entity)
	}

	/// Construct a new [`Action`] with the specified table and data.
	pub fn with_entry(table: &'a str, entry: &'a S) -> Self {
		let mut act = Self::new();

		act.set_table(table).set_entry(entry);

		act
	}
}

impl<'a, S: Entry, C: CrudOperation, T: OperationTarget> Debug for Action<'a, S, C, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut state = f.debug_struct("Action");

		state
			.field("kind", &self.kind())
			.field("target", &self.target());

		if let Some(key) = self.key() {
			state.field("key", &key);
		}

		if let Some(table) = self.table() {
			state.field("table", &table);
		}

		state.finish()
	}
}

impl<'a, S: Entry, C: CrudOperation, T: OperationTarget> Default for Action<'a, S, C, T> {
	fn default() -> Self {
		Self {
			inner: InnerAction::default(),
			kind: PhantomData,
			target: PhantomData,
		}
	}
}

unsafe impl<'a, S: Entry + Send, C: CrudOperation, T: OperationTarget> Send
	for Action<'a, S, C, T>
{
}

unsafe impl<'a, S: Entry + Sync, C: CrudOperation, T: OperationTarget> Sync
	for Action<'a, S, C, T>
{
}

impl<'a, S: Entry + Unpin, C: CrudOperation, T: OperationTarget> Unpin for Action<'a, S, C, T> {}

// Action run impls

impl<'a, S: Entry> CreateEntryAction<'a, S> {
	/// Validates and runs a [`CreateEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_entry`] fails, or if any of the [`Backend`] methods fail.
	pub fn run_create_entry<B: Backend>(
		self,
		chart: &'a Starchart<B>,
	) -> impl Future<Output = Result<(), ActionError>> + 'a {
		self.inner.create_entry(chart)
	}
}

impl<'a, S: Entry> ReadEntryAction<'a, S> {
	/// Validates and runs a [`ReadEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_key`] fails, or if any of the [`Backend`] methods fail.
	pub fn run_read_entry<B: Backend>(
		self,
		gateway: &'a Starchart<B>,
	) -> impl Future<Output = Result<Option<S>, ActionError>> + 'a {
		self.inner.read_entry(gateway)
	}
}

impl<'a, S: Entry> UpdateEntryAction<'a, S> {
	/// Validates and runs a [`UpdateEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_entry`] fails, or if any of the [`Backend`] methods fail.
	pub fn run_update_entry<B: Backend>(
		self,
		chart: &'a Starchart<B>,
	) -> impl Future<Output = Result<(), ActionError>> + 'a {
		self.inner.update_entry(chart)
	}
}

impl<'a, S: Entry> DeleteEntryAction<'a, S> {
	/// Validates and runs a [`DeleteEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_key`] fails, or if any of the [`Backend`] methods fail.
	pub fn run_delete_entry<B: Backend>(
		self,
		gateway: &'a Starchart<B>,
	) -> impl Future<Output = Result<bool, ActionError>> + 'a {
		self.inner.delete_entry(gateway)
	}
}

impl<'a, S: Entry> CreateTableAction<'a, S> {
	/// Validates and runs a [`CreateTableAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] fails, or if any of the [`Backend`] methods fail.
	pub fn run_create_table<B: Backend>(
		self,
		gateway: &'a Starchart<B>,
	) -> impl Future<Output = Result<(), ActionError>> + 'a {
		self.inner.create_table(gateway)
	}
}

impl<'a, S: Entry> ReadTableAction<'a, S> {
	/// Validates and runs a [`ReadTableAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] fails, or if any of the [`Backend`] methods fail.
	pub fn run_read_table<B: Backend, I>(
		self,
		gateway: &'a Starchart<B>,
	) -> impl Future<Output = Result<I, ActionError>> + 'a
	where
		I: FromIterator<S> + 'a,
	{
		self.inner.read_table(gateway)
	}
}

impl<'a, S: Entry> DeleteTableAction<'a, S> {
	/// Validates and runs a [`DeleteTableAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] fails, or if any of the [`Backend`] methods fail.
	pub fn run_delete_table<B: Backend>(
		self,
		gateway: &'a Starchart<B>,
	) -> impl Future<Output = Result<bool, ActionError>> + 'a {
		self.inner.delete_table(gateway)
	}
}
