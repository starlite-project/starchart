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
	future::Future,
	iter::FromIterator,
	marker::PhantomData,
	pin::Pin,
};

use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub use self::error::{ActionError, ActionRunError, ActionValidationError};
pub use self::{
	kind::ActionKind,
	r#impl::{
		ActionRunner, CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OpTarget,
		ReadOperation, TableTarget, UpdateOperation,
	},
	result::ActionResult,
	target::OperationTarget,
};
use crate::{backend::Backend, util::InnerUnwrap, Entry, IndexEntry, Key, Starchart};

#[cfg(all(feature = "metadata", not(tarpaulin_include)))]
const METADATA_KEY: &str = "__metadata__";

/// A type alias for an [`Action`] with [`CreateOperation`] and [`EntryTarget`] as the parameters.
pub type CreateEntryAction<S> = Action<S, CreateOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`ReadOperation`] and [`EntryTarget`] as the parameters.
pub type ReadEntryAction<S> = Action<S, ReadOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`EntryTarget`] as the parameters.
pub type UpdateEntryAction<S> = Action<S, UpdateOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`EntryTarget`] as the parameters.
pub type DeleteEntryAction<S> = Action<S, DeleteOperation, EntryTarget>;

/// A type alias for an [`Action`] with [`CreateOperation`] and [`TableTarget`] as the parameters.
pub type CreateTableAction<S> = Action<S, CreateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`ReadOperation`] and [`TableTarget`] as the parameters.
pub type ReadTableAction<S> = Action<S, ReadOperation, TableTarget>;

/// A type alias for an [`Action`] with [`UpdateOperation`] and [`TableTarget`] as the parameters.
///
/// This action can never been ran.
pub type UpdateTableAction<S> = Action<S, UpdateOperation, TableTarget>;

/// A type alias for an [`Action`] with [`DeleteOperation`] and [`TableTarget`] as the parameters.
pub type DeleteTableAction<S> = Action<S, DeleteOperation, TableTarget>;

/// An [`Action`] for easy [`CRUD`] operations within a [`Starchart`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Starchart`]: crate::Starchart
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Serialize, Deserialize)]
#[must_use = "an action alone has no side effects"]
pub struct Action<S, C: CrudOperation, T: OpTarget> {
	pub(crate) data: Option<Box<S>>,
	pub(crate) key: Option<String>,
	pub(crate) table: Option<String>,
	kind: PhantomData<C>,
	target: PhantomData<T>,
}

impl<S: Entry, C: CrudOperation, T: OpTarget> Action<S, C, T> {
	/// Creates a new [`Action`] with the specified operation.
	pub fn new() -> Self {
		Self {
			data: None,
			key: None,
			table: None,
			kind: PhantomData,
			target: PhantomData,
		}
	}

	/// Returns the [`ActionKind`] we will be performing with said action.
	#[allow(clippy::unused_self)]
	pub fn kind(&self) -> ActionKind {
		C::kind()
	}

	/// Returns the [`OperationTarget`] we will be performing with said action.
	#[must_use]
	#[allow(clippy::unused_self)]
	pub fn target(&self) -> OperationTarget {
		T::target()
	}

	/// Changes the [`CrudOperation`] of this [`Action`].
	pub fn into_operation<O: CrudOperation>(self) -> Action<S, O, T> {
		Action {
			data: self.data,
			key: self.key,
			table: self.table,
			kind: PhantomData,
			target: PhantomData,
		}
	}

	/// Changes the [`OpTarget`] of this [`Action`].
	pub fn into_target<T2: OpTarget>(self) -> Action<S, C, T2> {
		Action {
			data: self.data,
			key: self.key,
			table: self.table,
			kind: PhantomData,
			target: PhantomData,
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
			data: None,
			key: self.key,
			table: self.table,
			kind: PhantomData,
			target: PhantomData,
		}
	}

	/// Sets the table for this action.
	pub fn set_table(&mut self, table_name: &str) -> &mut Self {
		self.table = Some(table_name.to_owned());

		self // coverage:ignore-line
	}

	#[cfg(feature = "metadata")]
	#[cfg_attr(
		all(feature = "metadata", not(tarpaulin_include)),
		allow(clippy::future_not_send)
	)]
	async fn check_metadata<B: Backend>(
		&self,
		backend: &B,
		table_name: &str,
	) -> Result<(), ActionRunError<B::Error>> {
		backend
			.get::<S>(table_name, METADATA_KEY)
			.await // coverage:ignore-line
			.map(|_| {})
			.map_err(|_| ActionRunError::Metadata(type_name::<S>(), table_name.to_owned()))
	}

	pub fn validate_table(&self) -> Result<(), ActionValidationError> {
		if self.table.is_none() {
			return Err(ActionValidationError::Table);
		}

		#[cfg(feature = "metadata")]
		self.validate_metadata(self.table.as_deref())?;

		Ok(())
	}

	#[cfg(all(feature = "metadata", not(tarpaulin_include)))]
	#[allow(clippy::unused_self)]
	pub fn validate_metadata(&self, key: Option<&str>) -> Result<(), ActionValidationError> {
		if key == Some(METADATA_KEY) {
			return Err(ActionValidationError::Metadata);
		}

		Ok(())
	}
}

// Entry helpers
impl<S: Entry, C: CrudOperation> Action<S, C, EntryTarget> {
	/// Sets the key for the action.
	///
	/// Users should prefer to call [`Self::set_entry`] over this, as setting the
	/// entry will automatically call this.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.key = Some(key.to_key());

		self // coverage:ignore-line
	}

	/// Sets the data for the action.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_data(&mut self, entity: &S) -> &mut Self {
		self.data = Some(Box::new(entity.clone()));

		self // coverage:ignore-line
	}

	pub fn validate_key(&self) -> Result<(), ActionValidationError> {
		if self.key.is_none() {
			return Err(ActionValidationError::Key);
		}

		#[cfg(feature = "metadata")]
		self.validate_metadata(self.key.as_deref())?;

		Ok(())
	}

	pub fn validate_data(&self) -> Result<(), ActionValidationError> {
		if self.data.is_none() {
			return Err(ActionValidationError::Data);
		}

		Ok(())
	}

	pub fn validate_entry(&self) -> Result<(), ActionValidationError> {
		self.validate_key()?;
		self.validate_data()
	}
}

// Crud helpers
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

// Target helpers
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

// Combined helpers
impl<S: Entry> CreateTableAction<S> {
	/// Creates a new [`CreateOperation`] based [`TableTarget`] operation.
	pub fn create_table() -> Self {
		Self::new()
	}
}

impl<S: Entry> ReadTableAction<S> {
	/// Creates a new [`ReadOperation`] based [`TableTarget`] operation.
	pub fn read_table() -> Self {
		Self::new()
	}
}

// Update table is specifically omitted as it's unsupported

impl<S: Entry> DeleteTableAction<S> {
	/// Creates a new [`DeleteOperation`] based [`TableTarget`] operation.
	pub fn delete_table() -> Self {
		Self::new()
	}
}

impl<S: Entry> CreateEntryAction<S> {
	/// Creates a new [`CreateOperation`] based [`EntryTarget`] operation.
	pub fn create_entry() -> Self {
		Self::new()
	}
}

impl<S: Entry> ReadEntryAction<S> {
	/// Creates a new [`ReadOperation`] based [`EntryTarget`] operation.
	pub fn read_entry() -> Self {
		Self::new()
	}
}

impl<S: Entry> UpdateEntryAction<S> {
	/// Creates a new [`UpdateOperation`] based [`EntryTarget`] operation.
	pub fn update_entry() -> Self {
		Self::new()
	}
}

impl<S: Entry> DeleteEntryAction<S> {
	/// Creates a new [`DeleteOperation`] based [`EntryTarget`] operation.
	pub fn delete_entry() -> Self {
		Self::new()
	}
}

impl<S: IndexEntry, C: CrudOperation> Action<S, C, EntryTarget> {
	/// Sets the [`Entry`] and [`Key`] that this [`Action`] will act over.
	pub fn set_entry(&mut self, entity: &S) -> &mut Self {
		self.set_key(&entity.key()).set_data(entity)
	}
}

#[cfg(not(tarpaulin_include))]
impl<S: Entry, C: CrudOperation, T: OpTarget> Debug for Action<S, C, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Action")
			.field("kind", &self.kind())
			.field("table", &self.table)
			.field("data", &self.data)
			.field("key", &self.key)
			.field("target", &self.target())
			.finish()
	}
}

impl<S: Entry, C: CrudOperation, T: OpTarget> Clone for Action<S, C, T> {
	fn clone(&self) -> Self {
		Self {
			data: self.data.clone(),
			key: self.key.clone(),
			table: self.table.clone(),
			kind: PhantomData,
			target: PhantomData,
		}
	}
}

impl<S: Entry, C: CrudOperation, T: OpTarget> Default for Action<S, C, T> {
	fn default() -> Self {
		Self {
			data: None,
			key: None,
			table: None,
			kind: PhantomData,
			target: PhantomData,
		}
	}
}

unsafe impl<S: Entry + Send, C: CrudOperation, T: OpTarget> Send for Action<S, C, T> {}

unsafe impl<S: Entry + Sync, C: CrudOperation, T: OpTarget> Sync for Action<S, C, T> {}

// Action run impls

impl<S: Entry, C: CrudOperation, T: OpTarget> Action<S, C, T> {
	pub async fn run<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> ActionResult<S, ActionError<<B as Backend>::Error>> {
		match self.target() {
			OperationTarget::Entry => match self.kind() {
				ActionKind::Create => {
					let create_entry: CreateEntryAction<S> = self.into_target().into_operation();

					if let Err(e) = create_entry.run_create_entry(gateway).await {
						ActionResult::Error(e)
					} else {
						ActionResult::Create
					}
				}
				ActionKind::Read => {
					let read_entry: ReadEntryAction<S> = self.into_target().into_operation();

					match read_entry.run_read_entry(gateway).await {
						Ok(v) => ActionResult::ReadSingle(v),
						Err(e) => ActionResult::Error(e),
					}
				}
				ActionKind::Update => {
					let update_entry: UpdateEntryAction<S> = self.into_target().into_operation();
				}
				_ => todo!(),
			},
			_ => todo!(),
		}
	}
}

impl<S: Entry> CreateEntryAction<S> {
	/// Validates and runs a [`CreateEntryAction`].
	///
	/// # Errors
	///
	/// This returns an error if [`Self::validate_table`] or [`Self::validate_entry`] fails, or if any of the [`Backend`] methods fail.
	pub async fn run_create_entry<B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<(), ActionError<<B as Backend>::Error>> {
		self.validate_table()?;
		self.validate_entry()?;
		let lock = chart.guard.exclusive();
		unsafe { self.run_create_entry_unchecked(chart.backend()).await? };
		drop(lock);
		Ok(())
	}

	/// Runs a [`CreateEntryAction`], not checking for any validation issues beforehand.
	///
	/// # Errors
	///
	/// This will fail if any of the [`Backend`] methods fail.
	///
	/// # Safety
	/// This does not check any variants beforehand. Calling this without calling [`Self::set_table`], [`Self::set_data`],
	/// and [`Self::set_key`] (or [`Self::set_entry`] for the last two) will cause immediate `UB`.
	///
	/// Additionally, this does no atomic locking, so running this at the same time as a read or write will cause data races.
	pub async unsafe fn run_create_entry_unchecked<B: Backend>(
		mut self,
		backend: &B,
	) -> Result<(), ActionRunError<<B as Backend>::Error>> {
		let table_name = self.table.take().inner_unwrap();
		let key = self.key.take().inner_unwrap();
		let entry = self.data.take().inner_unwrap();
		if backend.has(&table_name, &key).await? {
			return Ok(());
		}

		#[cfg(feature = "metadata")]
		self.check_metadata(backend, &table_name).await?;

		backend.create(&table_name, &key, &*entry).await?;

		Ok(())
	}
}

impl<S: Entry> ReadEntryAction<S> {
	pub async fn run_read_entry<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<Option<S>, ActionError<<B as Backend>::Error>> {
		self.validate_table()?;
		self.validate_key()?;
		let lock = gateway.guard.exclusive();
		let res = unsafe { self.run_read_entry_unchecked(gateway.backend()).await? };
		drop(lock);
		Ok(res)
	}

	pub async unsafe fn run_read_entry_unchecked<B: Backend>(
		mut self,
		backend: &B,
	) -> Result<Option<S>, ActionRunError<<B as Backend>::Error>> {
		let table_name = self.table.take().inner_unwrap();
		let key = self.key.take().inner_unwrap();

		#[cfg(feature = "metadata")]
		self.check_metadata(backend, &table_name).await?;

		Ok(backend.get(&table_name, &key).await?)
	}
}

impl<S: Entry> UpdateEntryAction<S> {
	pub async fn run_update_entry<B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<(), ActionError<<B as Backend>::Error>> {
		self.validate_table()?;
		self.validate_entry()?;
		let lock = chart.guard.exclusive();
		unsafe { self.run_update_entry_unchecked(chart.backend()).await? };
		drop(lock);
		Ok(())
	}

	pub async unsafe fn run_update_entry_unchecked<B: Backend>(
		mut self,
		backend: &B,
	) -> Result<(), ActionRunError<<B as Backend>::Error>> {
		let table = self.table.take().inner_unwrap();
		let key = self.key.take().inner_unwrap();
		let new_data = self.data.take().inner_unwrap();

		#[cfg(feature = "metadata")]
		self.check_metadata(backend, &table).await?;

		backend.update(&table, &key, &new_data).await?;

		Ok(())
	}
}

impl<S: Entry> DeleteEntryAction<S> {
	pub async fn run_delete_entry<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<bool, ActionError<<B as Backend>::Error>> {
		self.validate_table()?;
		self.validate_key()?;
		let lock = gateway.guard.exclusive();
		let res = unsafe { self.run_delete_entry_unchecked(gateway.backend()).await? };
		drop(lock);
		Ok(res)
	}

	pub async unsafe fn run_delete_entry_unchecked<B: Backend>(
		mut self,
		backend: &B,
	) -> Result<bool, ActionRunError<<B as Backend>::Error>> {
		let table = self.table.take().inner_unwrap();
		let key = self.key.take().inner_unwrap();
		let exists = backend.has(&table, &key).await?;

		backend.delete(&table, &key).await?;

		let after_exists = backend.has(&table, &key).await?;

		Ok(exists != after_exists)
	}
}

impl<S: Entry> CreateTableAction<S> {
	pub async fn run_create_table<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<(), ActionError<<B as Backend>::Error>> {
		self.validate_table()?;
		let lock = gateway.guard.exclusive();
		unsafe { self.run_create_table_unchecked(gateway.backend()).await? };
		drop(lock);
		Ok(())
	}

	pub async unsafe fn run_create_table_unchecked<B: Backend>(
		self,
		backend: &B,
	) -> Result<(), ActionRunError<<B as Backend>::Error>> {
		let table = self.table.inner_unwrap();

		backend.ensure_table(&table).await?;

		#[cfg(feature = "metadata")]
		{
			let metadata = S::default();
			backend.ensure(&table, METADATA_KEY, &metadata).await?;
		}

		Ok(())
	}
}

impl<S: Entry> ReadTableAction<S> {
	pub async fn run_read_table<B: Backend, I>(
		self,
		gateway: &Starchart<B>,
	) -> Result<I, ActionError<<B as Backend>::Error>>
	where
		I: FromIterator<S>,
	{
		self.validate_table()?;
		let lock = gateway.guard.shared();
		let res = unsafe { self.run_read_table_unchecked(gateway.backend()).await? };
		drop(lock);
		Ok(res)
	}

	pub async unsafe fn run_read_table_unchecked<B: Backend, I>(
		mut self,
		backend: &B,
	) -> Result<I, ActionRunError<<B as Backend>::Error>>
	where
		I: FromIterator<S>,
	{
		let table = self.table.take().inner_unwrap();

		#[cfg(feature = "metadata")]
		self.check_metadata(backend, &table).await?;

		let keys = backend.get_keys::<Vec<_>>(&table).await?;

		#[cfg(feature = "metadata")]
		let keys = keys
			.into_iter()
			.filter(|value| value != METADATA_KEY)
			.collect::<Vec<_>>();

		let keys_borrowed = keys.iter().map(String::as_str).collect::<Vec<_>>();

		let data = backend.get_all::<S, I>(&table, &keys_borrowed).await?;

		Ok(data)
	}
}

impl<S: Entry> DeleteTableAction<S> {
	pub async fn run_delete_table<B: Backend>(
		self,
		gateway: &Starchart<B>,
	) -> Result<bool, ActionError<<B as Backend>::Error>> {
		self.validate_table()?;
		let lock = gateway.guard.exclusive();
		let res = unsafe { self.run_delete_table_unchecked(gateway.backend()).await? };
		drop(lock);
		Ok(res)
	}

	pub async unsafe fn run_delete_table_unchecked<B: Backend>(
		self,
		backend: &B,
	) -> Result<bool, ActionRunError<<B as Backend>::Error>> {
		let table = self.table.inner_unwrap();

		let exists = backend.has_table(&table).await?;
		backend.delete_table(&table).await?;
		let new_exists = backend.has_table(&table).await?;

		Ok(exists != new_exists)
	}
}

#[cfg(all(test, feature = "memory", feature = "metadata"))]
mod tests {
	use serde::{Deserialize, Serialize};

	use super::{
		error::ActionError, Action, ActionKind, CreateEntryAction, CreateOperation,
		CreateTableAction, DeleteEntryAction, DeleteOperation, DeleteTableAction, EntryTarget,
		OperationTarget, ReadEntryAction, ReadOperation, ReadTableAction, TableTarget,
		UpdateEntryAction, UpdateOperation,
	};
	use crate::{
		action::ActionRunError, backend::MemoryBackend, error::MemoryError, IndexEntry, Starchart,
	};

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
		let action: CreateTableAction<Settings> = Action::new().set_table("table").clone();

		todo!()
	}

	#[test]
	fn new_kind_and_target() {
		let action: Action<Settings, ReadOperation, EntryTarget> = Action::new();

		assert_eq!(action.kind(), ActionKind::Read);
		assert_eq!(action.target(), OperationTarget::Entry);
	}

	#[test]
	fn into_operation_and_target() {
		let action: Action<Settings, ReadOperation, EntryTarget> = Action::new();

		assert_eq!(action.kind(), ActionKind::Read);
		let new_action = action.into_operation::<CreateOperation>();
		assert_eq!(new_action.kind(), ActionKind::Create);

		assert_eq!(new_action.target(), OperationTarget::Entry);
		let new_target = new_action.into_target::<TableTarget>();
		assert_eq!(new_target.target(), OperationTarget::Table);
	}

	#[test]
	fn conversion_methods() {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
		struct NewSettings;

		let action: Action<Settings, ReadOperation, EntryTarget> = Action::new();

		let create = action.into_create();
		assert_eq!(create.kind(), ActionKind::Create);

		let read = create.into_read();
		assert_eq!(read.kind(), ActionKind::Read);

		let update = read.into_update();
		assert_eq!(update.kind(), ActionKind::Update);

		let delete = update.into_delete();
		assert_eq!(delete.kind(), ActionKind::Delete);

		let table = delete.into_table();
		assert_eq!(table.target(), OperationTarget::Table);

		let entry = table.into_entry();
		assert_eq!(entry.target(), OperationTarget::Entry);

		let new_entry = entry.with_entry::<NewSettings>();
		assert!(new_entry.data.is_none());
	}

	#[test]
	fn set_methods() {
		let mut action: Action<Settings, ReadOperation, EntryTarget> =
			Action::read_entry().set_entry(&Settings::default()).clone();

		assert_eq!(action.data, Some(Box::new(Settings::default())));
		assert_eq!(action.key, Some(String::from("0")));

		let mut action = action.set_key(&"1").clone();
		assert_eq!(action.key, Some(String::from("1")));

		let action = action.set_data(&Settings {
			id: 7,
			option: false,
			value: 79,
		});

		assert_eq!(
			action.data,
			Some(Box::new(Settings {
				id: 7,
				option: false,
				value: 79
			}))
		);
	}

	#[test]
	fn constructor_methods() {
		assert_eq!(
			Action::<Settings, CreateOperation, EntryTarget>::create().kind(),
			ActionKind::Create
		);
		assert_eq!(
			Action::<Settings, ReadOperation, EntryTarget>::read().kind(),
			ActionKind::Read
		);
		assert_eq!(
			Action::<Settings, UpdateOperation, EntryTarget>::update().kind(),
			ActionKind::Update
		);
		assert_eq!(
			Action::<Settings, DeleteOperation, EntryTarget>::delete().kind(),
			ActionKind::Delete
		);
		assert_eq!(
			Action::<Settings, ReadOperation, TableTarget>::table().target(),
			OperationTarget::Table
		);
		assert_eq!(
			Action::<Settings, ReadOperation, EntryTarget>::entry().target(),
			OperationTarget::Entry
		);

		let create_table: Action<Settings, CreateOperation, TableTarget> = Action::create_table();
		assert_eq!(
			(create_table.kind(), create_table.target()),
			(ActionKind::Create, OperationTarget::Table)
		);

		let read_entry: Action<Settings, ReadOperation, EntryTarget> = Action::read_entry();
		assert_eq!(
			(read_entry.kind(), read_entry.target()),
			(ActionKind::Read, OperationTarget::Entry)
		);

		let update_entry: Action<Settings, UpdateOperation, EntryTarget> = Action::update_entry();
		assert_eq!(
			(update_entry.kind(), update_entry.target()),
			(ActionKind::Update, OperationTarget::Entry)
		);

		let delete_entry: Action<Settings, DeleteOperation, EntryTarget> = Action::delete_entry();
		assert_eq!(
			(delete_entry.kind(), delete_entry.target()),
			(ActionKind::Delete, OperationTarget::Entry)
		);

		let read_table: Action<Settings, ReadOperation, TableTarget> = Action::read_table();
		assert_eq!(
			(read_table.kind(), read_table.target()),
			(ActionKind::Read, OperationTarget::Table)
		);

		let delete_table: Action<Settings, DeleteOperation, TableTarget> = Action::delete_table();
		assert_eq!(
			(delete_table.kind(), delete_table.target()),
			(ActionKind::Delete, OperationTarget::Table)
		);

		let create_entry: Action<Settings, CreateOperation, EntryTarget> = Action::create_entry();
		assert_eq!(
			(create_entry.kind(), create_entry.target()),
			(ActionKind::Create, OperationTarget::Entry)
		);
	}

	#[test]
	fn default() {
		let default = Action::<Settings, ReadOperation, EntryTarget>::default();

		assert!(default.data.is_none());
		assert!(default.key.is_none());
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
		action.set_data(&Settings::default());
		assert!(action.validate_data().is_ok());

		action.set_key(&"__metadata__");
		assert!(action.validate_key().is_err());

		action.set_table("__metadata__");
		assert!(action.validate_table().is_err());
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn basic_run() -> Result<(), ActionError<MemoryError>> {
		let gateway = setup_gateway().await;
		let mut action: Action<Settings, CreateOperation, TableTarget> = Action::new();

		action.set_table("table");

		// gateway.run(action).await??;

		action.run(&gateway).await;

		for i in 0..3 {
			let settings = Settings {
				id: i,
				option: false,
				value: 8,
			};
			let mut action: CreateEntryAction<Settings> = Action::new();

			action.set_table("table").set_entry(&settings);

			gateway.run(action).await??;
		}

		let mut read_table: ReadTableAction<Settings> = Action::new();

		read_table.set_table("table");

		let mut values: Vec<Settings> = gateway.run(read_table).await??;
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
	async fn duplicate_creates() -> Result<(), ActionError<MemoryError>> {
		let gateway = setup_gateway().await;
		setup_table(&gateway).await;

		let mut create_action: CreateEntryAction<Settings> = Action::new();

		create_action
			.set_table("table")
			.set_entry(&Settings::default());

		let double_create = create_action.clone();

		assert!(gateway.run(create_action).await?.is_ok());

		assert!(gateway.run(double_create).await?.is_ok());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn read_and_update() -> Result<(), ActionError<MemoryError>> {
		let gateway = setup_gateway().await;
		setup_table(&gateway).await;
		{
			let mut create_action: CreateEntryAction<Settings> = Action::new();
			create_action
				.set_table("table")
				.set_entry(&Settings::default());
			gateway.run(create_action).await??;
		}

		let mut read_action: ReadEntryAction<Settings> = Action::new();

		read_action.set_key(&0_u32).set_table("table");

		let reread_action = read_action.clone();

		let value = gateway.run(read_action).await??;
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

		gateway.run(update_action).await??;

		assert_eq!(gateway.run(reread_action).await??, Some(new_settings));

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn deletes() -> Result<(), ActionError<MemoryError>> {
		let gateway = setup_gateway().await;
		setup_table(&gateway).await;

		{
			let mut create_action = CreateEntryAction::<Settings>::new();

			create_action
				.set_table("table")
				.set_entry(&Settings::default());

			gateway.run(create_action).await??;
		}

		let mut delete_action: DeleteEntryAction<Settings> = Action::new();
		delete_action.set_table("table").set_key(&0_u32);
		assert!(gateway.run(delete_action).await??);
		let mut read_action: ReadEntryAction<Settings> = Action::new();
		read_action.set_table("table").set_key(&0_u32);
		assert_eq!(gateway.run(read_action).await??, None);

		let mut delete_table_action: DeleteTableAction<Settings> = Action::new();
		delete_table_action.set_table("table");
		assert!(gateway.run(delete_table_action).await??);
		let mut read_table: ReadTableAction<Settings> = Action::new();
		read_table.set_table("table");

		let res: Result<Vec<_>, ActionRunError<MemoryError>> = gateway.run(read_table).await?;

		assert!(res.is_err());

		Ok(())
	}
}
