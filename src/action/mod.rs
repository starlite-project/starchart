//! The action structs for CRUD operations.

// TODO: Add overwrite option.

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
use futures_util::{future::ok, Future};

#[doc(hidden)]
pub use self::error::{
	ActionError, ActionErrorType, ActionRunError, ActionRunErrorType, ActionValidationError,
	ActionValidationErrorType,
};
pub use self::{
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
#[cfg(feature = "action")]
pub type CreateEntryAction<'a, S> = Action<'a, S, CreateOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`ReadOperation`] and [`EntryTarget`] as the parameters.
#[cfg(feature = "action")]
pub type ReadEntryAction<'a, S> = Action<'a, S, ReadOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`EntryTarget`] as the parameters.
#[cfg(feature = "action")]
pub type UpdateEntryAction<'a, S> = Action<'a, S, UpdateOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`EntryTarget`] as the parameters.
#[cfg(feature = "action")]
pub type DeleteEntryAction<'a, S> = Action<'a, S, DeleteOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`CreateOperation`] and [`TableTarget`] as the parameters.
#[cfg(feature = "action")]
pub type CreateTableAction<'a, S> = Action<'a, S, CreateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`ReadOperation`] and [`TableTarget`] as the parameters.
#[cfg(feature = "action")]
pub type ReadTableAction<'a, S> = Action<'a, S, ReadOperation, TableTarget>;

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`TableTarget`] as the parameters.
///
/// This action can never been ran.
#[cfg(feature = "action")]
pub type UpdateTableAction<'a, S> = Action<'a, S, UpdateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`TableTarget`] as the parameters.
#[cfg(feature = "action")]
pub type DeleteTableAction<'a, S> = Action<'a, S, DeleteOperation, TableTarget>;

#[derive(Debug, Clone)]
pub(crate) struct InnerAction<'a, S> {
	pub data: Option<&'a S>,
	pub key: Option<String>,
	pub table: Option<&'a str>,
}

impl<'a, S> InnerAction<'a, S> {
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

impl<'a, S: Entry> InnerAction<'a, S> {
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
		async { Ok(()) }
	}

	async fn create_entry<B: Backend>(mut self, chart: &Starchart<B>) -> Result<(), ActionError> {
		self.validate_entry()?;
		self.validate_table()?;

		let lock = chart.guard.exclusive();

		let backend = &**chart;

		let (table, key, entry) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
				self.data.take().inner_unwrap(),
			)
		};

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

		let lock = chart.guard.shared();

		let backend = &**chart;

		let (table, key) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
			)
		};

		self.check_metadata(backend, table).await?;

		let res = backend
			.get(table, &key)
			.await
			.map_err(|e| ActionRunError {
				source: Some(Box::new(e)),
				kind: ActionRunErrorType::Backend,
			})?;

		drop(lock);

		Ok(res)
	}

	async fn update_entry<B: Backend>(mut self, chart: &Starchart<B>) -> Result<(), ActionError> {
		self.validate_table()?;
		self.validate_entry()?;

		let lock = chart.guard.exclusive();

		let backend = &**chart;

		let (table, key, entry) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
				self.data.take().inner_unwrap(),
			)
		};

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
		let lock = chart.guard.exclusive();

		let backend = &**chart;

		let (table, key) = unsafe {
			(
				self.table.take().inner_unwrap(),
				self.key.take().inner_unwrap(),
			)
		};

		self.check_metadata(backend, table).await?;

		if !backend
			.has(table, &key)
			.await
			.map_err(|e| ActionRunError {
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

		let lock = chart.guard.exclusive();

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
		let lock = chart.guard.shared();

		let backend = &**chart;

		let table = unsafe { self.table.take().inner_unwrap() };

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

		let lock = chart.guard.exclusive();

		let backend = &**chart;

		let table = unsafe { self.table.take().inner_unwrap() };

		self.check_metadata(backend, table).await?;

		if !backend
			.has_table(table)
			.await
			.map_err(|e| ActionRunError {
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

impl<'a, S> Default for InnerAction<'a, S> {
	fn default() -> Self {
		Self::new()
	}
}

/// An [`Action`] for easy [`CRUD`] operations within a [`Starchart`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Starchart`]: crate::Starchart
#[derive(Clone)]
#[must_use = "an action alone has no side effects"]
#[cfg(feature = "action")]
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
	#[must_use]
	#[allow(clippy::unused_self)]
	pub fn target(&self) -> TargetKind {
		T::target()
	}

	/// Sets the table for this action.
	pub fn set_table(&mut self, table_name: &'a str) -> &mut Self {
		self.inner.table = Some(table_name);

		self // coverage:ignore-line
	}

	/// Validates that the table key is set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_table`] has not yet been called.
	#[inline]
	pub fn validate_table(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_table()
	}

	/// Validates that the key is not the private metadata key.
	///
	/// Does nothing if the `metadata` feature is not enabled.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_key`] was passed the private metadata key.
	#[allow(clippy::unused_self)]
	#[inline]
	pub fn validate_metadata(&self, key: Option<&str>) -> Result<(), ActionValidationError> {
		self.inner.validate_metadata(key)
	}
}

// Entry helpers
impl<'a, S: Entry, C: CrudOperation> Action<'a, S, C, EntryTarget> {
	/// Sets the key for the action.
	///
	/// Users should prefer to call [`Self::set_entry`] over this, as setting the
	/// entry will automatically call this.
	///
	/// This is unused on [`TargetKind::Table`] actions.
	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.inner.key = Some(key.to_key());

		self // coverage:ignore-line
	}

	/// Sets the data for the action.
	///
	/// This is unused on [`TargetKind::Table`] actions.
	pub fn set_data(&mut self, entity: &'a S) -> &mut Self {
		self.inner.data = Some(entity);

		self // coverage:ignore-line
	}

	/// Validate that the key has been set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_key`] has not yet been called.
	#[inline]
	pub fn validate_key(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_key()
	}

	/// Validates that the data has been set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_data`] has not yet been called.
	#[inline]
	pub fn validate_data(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_data()
	}

	/// Validates that both the key and data have been set.
	///
	/// # Errors
	///
	/// This errors if both the [`Self::set_key`] and [`Self::set_data`] (or [`Self::set_entry`]) has not been called.
	#[inline]
	pub fn validate_entry(&self) -> Result<(), ActionValidationError> {
		self.inner.validate_entry()
	}
}

// Combined helpers
impl<'a, S: Entry> CreateTableAction<'a, S> {
	/// Creates a new [`CreateOperation`] based [`TableTarget`] operation.
	pub fn create_table() -> Self {
		Self::new()
	}
}

impl<'a, S: Entry> ReadTableAction<'a, S> {
	/// Creates a new [`ReadOperation`] based [`TableTarget`] operation.
	pub fn read_table() -> Self {
		Self::new()
	}
}

// Update table is specifically omitted as it's unsupported

impl<'a, S: Entry> DeleteTableAction<'a, S> {
	/// Creates a new [`DeleteOperation`] based [`TableTarget`] operation.
	pub fn delete_table() -> Self {
		Self::new()
	}
}

impl<'a, S: Entry> CreateEntryAction<'a, S> {
	/// Creates a new [`CreateOperation`] based [`EntryTarget`] operation.
	pub fn create_entry() -> Self {
		Self::new()
	}
}

impl<'a, S: Entry> ReadEntryAction<'a, S> {
	/// Creates a new [`ReadOperation`] based [`EntryTarget`] operation.
	pub fn read_entry() -> Self {
		Self::new()
	}
}

impl<'a, S: Entry> UpdateEntryAction<'a, S> {
	/// Creates a new [`UpdateOperation`] based [`EntryTarget`] operation.
	pub fn update_entry() -> Self {
		Self::new()
	}
}

impl<'a, S: Entry> DeleteEntryAction<'a, S> {
	/// Creates a new [`DeleteOperation`] based [`EntryTarget`] operation.
	pub fn delete_entry() -> Self {
		Self::new()
	}
}

impl<'a, S: IndexEntry, C: CrudOperation> Action<'a, S, C, EntryTarget>
where
	<S as IndexEntry>::Key: 'a,
{
	/// Sets the [`Entry`] and [`Key`] that this [`Action`] will act over.
	pub fn set_entry(&mut self, entity: &'a S) -> &mut Self
	{
		self.set_key(&entity.key().to_key()).set_data(entity)
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

impl<'a, S: Entry, C: CrudOperation, T: OperationTarget> Action<'a, S, C, T> {
	/// Runs an [`Action`] to completion.
	///
	/// This method will dispatch to whatever method is needed for the action, and shouldn't be used directly if it can be helped.
	///
	/// # Errors
	///
	/// This will error if any of the other `run` prefixed methods cause an error.
	///
	/// # Panics
	///
	/// This method will panic if [`CrudOperation`] is [`UpdateOperation`] and [`OperationTarget`] is [`TableTarget`], as updating tables is not currently supported.
	#[inline]
	pub async fn run<B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<ActionResult<S>, ActionError> {
		match (self.kind(), self.target()) {
			(ActionKind::Create, TargetKind::Entry) => {
				self.inner.create_entry(chart).await?;
				Ok(ActionResult::Create)
			}
			(ActionKind::Read, TargetKind::Entry) => {
				let res = self.inner.read_entry(chart).await?;
				Ok(ActionResult::SingleRead(res))
			}
			(ActionKind::Update, TargetKind::Entry) => {
				self.inner.update_entry(chart).await?;
				Ok(ActionResult::Update)
			}
			(ActionKind::Delete, TargetKind::Entry) => {
				let res = self.inner.delete_entry(chart).await?;
				Ok(ActionResult::Delete(res))
			}
			(ActionKind::Create, TargetKind::Table) => {
				self.inner.create_table(chart).await?;
				Ok(ActionResult::Create)
			}
			(ActionKind::Read, TargetKind::Table) => {
				let res = self.inner.read_table(chart).await?;
				Ok(ActionResult::MultiRead(res))
			}
			(ActionKind::Update, TargetKind::Table) => panic!("updating tables is not supported"),
			(ActionKind::Delete, TargetKind::Table) => {
				let res = self.inner.delete_table(chart).await?;
				Ok(ActionResult::Delete(res))
			}
		}
	}
}

impl<'a, S: Entry> CreateEntryAction<'a, S> {
	/// Validates and runs a [`CreateEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_entry`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_create_entry<B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<(), ActionError> {
		self.inner.create_entry(chart).await
	}
}

impl<'a, S: Entry> ReadEntryAction<'a, S> {
	/// Validates and runs a [`ReadEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_key`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_read_entry<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<Option<S>, ActionError> {
		self.inner.read_entry(gateway).await
	}
}

impl<'a, S: Entry> UpdateEntryAction<'a, S> {
	/// Validates and runs a [`UpdateEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_entry`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_update_entry<B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<(), ActionError> {
		self.inner.update_entry(chart).await
	}
}

impl<'a, S: Entry> DeleteEntryAction<'a, S> {
	/// Validates and runs a [`DeleteEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_key`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_delete_entry<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<bool, ActionError> {
		self.inner.delete_entry(gateway).await
	}
}

impl<'a, S: Entry> CreateTableAction<'a, S> {
	/// Validates and runs a [`CreateTableAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_create_table<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<(), ActionError> {
		self.inner.create_table(gateway).await
	}
}

impl<'a, S: Entry> ReadTableAction<'a, S> {
	/// Validates and runs a [`ReadTableAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_read_table<B: Backend, I>(
		self,
		gateway: &Starchart<B>,
	) -> Result<I, ActionError>
	where
		I: FromIterator<S>,
	{
		self.inner.read_table(gateway).await
	}
}

impl<'a, S: Entry> DeleteTableAction<'a, S> {
	/// Validates and runs a [`DeleteTableAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_delete_table<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<bool, ActionError> {
		self.inner.delete_table(gateway).await
	}
}

#[cfg(all(test, feature = "memory", feature = "metadata"))]
mod tests {
	use serde::{Deserialize, Serialize};

	use super::{
		error::ActionError, Action, ActionKind, CreateEntryAction, CreateOperation,
		CreateTableAction, DeleteEntryAction, DeleteOperation, DeleteTableAction, EntryTarget,
		ReadEntryAction, ReadOperation, ReadTableAction, TableTarget, UpdateEntryAction,
		UpdateOperation,
	};
	use crate::{action::TargetKind, backend::MemoryBackend, IndexEntry, Starchart};

	#[derive(
		Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
	)]
	struct Settings {
		id: u64,
		option: bool,
		value: u8,
	}

	impl IndexEntry for Settings {
		type Key = u64;

		fn key(&self) -> Self::Key {
			self.id
		}
	}

	async fn setup_gateway() -> Starchart<MemoryBackend> {
		let backend = MemoryBackend::new();
		Starchart::new(backend).await.unwrap()
	}

	async fn setup_table(gateway: &Starchart<MemoryBackend>) {
		let mut action: CreateTableAction<Settings> = Action::new();

		action.set_table("table");

		action.run_create_table(gateway).await.unwrap();
	}

	#[test]
	fn new_kind_and_target() {
		let action: Action<Settings, ReadOperation, EntryTarget> = Action::new();

		assert_eq!(action.kind(), ActionKind::Read);
		assert_eq!(action.target(), TargetKind::Entry);
	}

	#[test]
	fn set_methods() {
		let def = Settings::default();
		let mut action: Action<Settings, ReadOperation, EntryTarget> =
			Action::read_entry().set_entry(&def).clone();

		assert_eq!(action.data(), Some(&Settings::default()));
		assert_eq!(action.key(), Some("0"));

		let mut action = action.set_key(&"1").clone();
		assert_eq!(action.key(), Some("1"));

		let action = action.set_data(&Settings {
			id: 7,
			option: false,
			value: 79,
		});

		assert_eq!(
			action.data(),
			Some(&Settings {
				id: 7,
				option: false,
				value: 79
			})
		);
	}

	#[test]
	fn constructor_methods() {
		let create_table: Action<Settings, CreateOperation, TableTarget> = Action::create_table();
		assert_eq!(
			(create_table.kind(), create_table.target()),
			(ActionKind::Create, TargetKind::Table)
		);

		let read_entry: Action<Settings, ReadOperation, EntryTarget> = Action::read_entry();
		assert_eq!(
			(read_entry.kind(), read_entry.target()),
			(ActionKind::Read, TargetKind::Entry)
		);

		let update_entry: Action<Settings, UpdateOperation, EntryTarget> = Action::update_entry();
		assert_eq!(
			(update_entry.kind(), update_entry.target()),
			(ActionKind::Update, TargetKind::Entry)
		);

		let delete_entry: Action<Settings, DeleteOperation, EntryTarget> = Action::delete_entry();
		assert_eq!(
			(delete_entry.kind(), delete_entry.target()),
			(ActionKind::Delete, TargetKind::Entry)
		);

		let read_table: Action<Settings, ReadOperation, TableTarget> = Action::read_table();
		assert_eq!(
			(read_table.kind(), read_table.target()),
			(ActionKind::Read, TargetKind::Table)
		);

		let delete_table: Action<Settings, DeleteOperation, TableTarget> = Action::delete_table();
		assert_eq!(
			(delete_table.kind(), delete_table.target()),
			(ActionKind::Delete, TargetKind::Table)
		);

		let create_entry: Action<Settings, CreateOperation, EntryTarget> = Action::create_entry();
		assert_eq!(
			(create_entry.kind(), create_entry.target()),
			(ActionKind::Create, TargetKind::Entry)
		);
	}

	#[test]
	fn default() {
		let default = Action::<Settings, ReadOperation, EntryTarget>::default();

		assert!(default.data().is_none());
		assert!(default.key().is_none());
	}

	#[test]
	fn validation_methods() {
		let mut action: Action<Settings, ReadOperation, EntryTarget> = Action::default();

		assert!(action.validate_key().is_err());
		action.set_key(&"foo");
		assert!(action.validate_key().is_ok());

		assert!(action.validate_table().is_err());
		action.set_table("test");
		assert!(action.validate_table().is_ok());

		assert!(action.validate_data().is_err());
		let def = Settings::default();
		action.set_data(&def);
		assert!(action.validate_data().is_ok());

		action.set_key(&"__metadata__");
		assert!(action.validate_key().is_err());

		action.set_table("__metadata__");
		assert!(action.validate_table().is_err());
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn basic_run() -> Result<(), ActionError> {
		let gateway = setup_gateway().await;
		let mut action: Action<Settings, CreateOperation, TableTarget> = Action::new();

		action.set_table("table");

		action.run(&gateway).await?.unwrap_create();

		for i in 0..3 {
			let settings = Settings {
				id: i,
				option: false,
				value: 8,
			};
			let mut action: CreateEntryAction<Settings> = Action::new();

			action.set_table("table").set_entry(&settings);

			action.run(&gateway).await?.unwrap_create();
		}

		let mut read_table: ReadTableAction<Settings> = Action::new();

		read_table.set_table("table");

		let mut values = read_table
			.run(&gateway)
			.await?
			.unwrap_multi_read::<Vec<_>>();
		let mut expected = vec![
			Settings {
				id: 0,
				option: false,
				value: 8,
			},
			Settings {
				id: 1,
				option: false,
				value: 8,
			},
			Settings {
				id: 2,
				option: false,
				value: 8,
			},
		];

		values.sort();
		expected.sort();

		assert_eq!(values, expected);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn duplicate_creates() -> Result<(), ActionError> {
		let gateway = setup_gateway().await;
		setup_table(&gateway).await;

		let mut create_action: CreateEntryAction<Settings> = Action::new();
		let def = Settings::default();

		create_action
			.set_table("table")
			.set_entry(&def);

		let double_create = create_action.clone();

		assert!(create_action.run(&gateway).await.is_ok());

		assert!(double_create.run(&gateway).await.is_ok());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn read_and_update() -> Result<(), ActionError> {
		let gateway = setup_gateway().await;
		setup_table(&gateway).await;
		{
			let def = Settings::default();
			let mut create_action: CreateEntryAction<Settings> = Action::new();
			create_action
				.set_table("table")
				.set_entry(&def);
			// gateway.run(create_action).await??;
			create_action.run(&gateway).await?.unwrap_create();
		}

		let mut read_action: ReadEntryAction<Settings> = Action::new();

		read_action.set_key(&0_u32).set_table("table");

		let reread_action = read_action.clone();

		let value = read_action.run(&gateway).await?.unwrap_single_read();
		assert_eq!(value, Some(Settings::default()));

		let new_settings = Settings {
			id: 0,
			option: true,
			value: 42,
		};

		let mut update_action: UpdateEntryAction<Settings> = Action::new();

		update_action
			.set_table("table")
			.set_key(&0_u32)
			.set_data(&new_settings);

		update_action.run(&gateway).await?.unwrap_update();

		assert_eq!(
			reread_action.run(&gateway).await?.unwrap_single_read(),
			Some(new_settings)
		);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn deletes() -> Result<(), ActionError> {
		let gateway = setup_gateway().await;
		setup_table(&gateway).await;

		{
			let mut create_action = CreateEntryAction::<Settings>::new();
			let def = Settings::default();

			create_action
				.set_table("table")
				.set_entry(&def);

			create_action.run(&gateway).await?.unwrap_create();
		}

		let mut delete_action: DeleteEntryAction<Settings> = Action::new();
		delete_action.set_table("table").set_key(&0_u32);
		assert!(delete_action.run(&gateway).await?.unwrap_delete());
		let mut read_action: ReadEntryAction<Settings> = Action::new();
		read_action.set_table("table").set_key(&0_u32);
		assert_eq!(read_action.run(&gateway).await?.unwrap_single_read(), None);

		let mut delete_table_action: DeleteTableAction<Settings> = Action::new();
		delete_table_action.set_table("table");
		// assert!(gateway.run(delete_table_action).await??);
		assert!(delete_table_action.run(&gateway).await?.unwrap_delete());
		let mut read_table: ReadTableAction<Settings> = Action::new();
		read_table.set_table("table");

		let res = read_table.run(&gateway).await;

		assert!(res.is_err());

		Ok(())
	}
}
