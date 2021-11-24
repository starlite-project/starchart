//! The action structs for CRUD operations.

// TODO: Add overwrite option.

mod error;
mod r#impl;
mod kind;
mod target;

use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	future::Future,
	marker::PhantomData,
	pin::Pin,
};

use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub use self::error::{ActionError, ActionRunError, ActionValidationError};
#[doc(inline)]
pub use self::{
	kind::ActionKind,
	r#impl::{
		ActionRunner, CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OpTarget,
		ReadOperation, TableTarget, UpdateOperation,
	},
	target::OperationTarget,
};
use crate::{backend::Backend, Entry, Gateway, IndexEntry, Key};

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

/// An [`Action`] for easy [`CRUD`] operations within a [`Gateway`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Gateway`]: crate::Gateway
#[derive(Serialize, Deserialize)]
#[must_use = "an action alone has no side effects"]
pub struct Action<S, C: CrudOperation, T: OpTarget> {
	data: Option<Box<S>>,
	key: Option<String>,
	table: Option<String>,
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

	fn validate_table(&self) -> Result<(), ActionValidationError> {
		if self.table.is_none() {
			return Err(ActionValidationError::Table);
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

	fn validate_key(&self) -> Result<(), ActionValidationError> {
		if self.key.is_none() {
			return Err(ActionValidationError::Key);
		}

		Ok(())
	}

	fn validate_data(&self) -> Result<(), ActionValidationError> {
		if self.data.is_none() {
			return Err(ActionValidationError::Data);
		}

		Ok(())
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
impl<S: Entry + Debug, C: CrudOperation, T: OpTarget> Debug for Action<S, C, T> {
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

impl<S: Entry + Clone, C: CrudOperation, T: OpTarget> Clone for Action<S, C, T> {
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

impl<B: Backend, S: Entry + 'static> ActionRunner<B, (), ActionRunError<B::Error>>
	for Action<S, CreateOperation, EntryTarget>
{
	unsafe fn run(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionRunError<B::Error>>> + Send + '_>> {
		// Create the lock outside of the async block, as the guard is invalid if created in async context.
		let lock = gateway.guard.write();
		let res = Box::pin(async move {
			// SAFETY: Action::validate should be called beforehand.
			let table_name = self.table.unwrap_unchecked();

			let key = self.key.unwrap_unchecked();

			let entry = self.data.unwrap_unchecked();

			let backend = gateway.backend();

			if backend.has(&table_name, &key).await? {
				return Ok(());
			}

			backend.create(&table_name, &key, &*entry).await?;

			Ok(())
		});

		RwLockWriteGuard::unlock_fair(lock);
		res
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_table()?;
		self.validate_key()
	}
}

impl<B: Backend, S: Entry + 'static> ActionRunner<B, Option<S>, ActionRunError<B::Error>>
	for Action<S, ReadOperation, EntryTarget>
{
	unsafe fn run(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<Option<S>, ActionRunError<B::Error>>> + Send + '_>> {
		let lock = gateway.guard.read();
		let res = Box::pin(async move {
			let table_name = self.table.unwrap_unchecked();

			let key = self.key.unwrap_unchecked();

			let backend = gateway.backend();

			Ok(backend.get(&table_name, &key).await?)
		});

		RwLockReadGuard::unlock_fair(lock);
		res
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_table()?;
		self.validate_key()
	}
}

impl<B: Backend, S: Entry + 'static> ActionRunner<B, (), ActionRunError<B::Error>>
	for Action<S, UpdateOperation, EntryTarget>
{
	unsafe fn run(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionRunError<B::Error>>> + Send + '_>> {
		let lock = gateway.guard.write();
		let res = Box::pin(async move {
			let table = self.table.unwrap_unchecked();

			let key = self.key.unwrap_unchecked();
			let new_data = self.data.unwrap_unchecked();

			let backend = gateway.backend();

			backend.update(&table, &key, &new_data).await?;

			Ok(())
		});

		RwLockWriteGuard::unlock_fair(lock);
		res
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_table()?;
		self.validate_key()?;
		self.validate_data()
	}
}

impl<B: Backend, S: Entry + 'static> ActionRunner<B, bool, ActionRunError<B::Error>>
	for Action<S, DeleteOperation, EntryTarget>
{
	unsafe fn run(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<bool, ActionRunError<B::Error>>> + Send + '_>> {
		let lock = gateway.guard.write();
		let res = Box::pin(async move {
			let table = self.table.unwrap_unchecked();

			let key = self.key.unwrap_unchecked();

			let backend = gateway.backend();

			let exists = backend.has(&table, &key).await?;

			backend.delete(&table, &key).await?;

			let after_exists = backend.has(&table, &key).await?;

			Ok(exists != after_exists)
		});

		RwLockWriteGuard::unlock_fair(lock);
		res
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_table()?;
		self.validate_key()
	}
}

// this is only here to satisfy the `clippy::type_complexity` lint
type ReadTableResult<'a, B, S> =
	Pin<Box<dyn Future<Output = Result<Vec<S>, ActionRunError<B>>> + Send + 'a>>;

impl<B: Backend, S: Entry + 'static> ActionRunner<B, Vec<S>, ActionRunError<B::Error>>
	for Action<S, ReadOperation, TableTarget>
{
	unsafe fn run(self, gateway: &Gateway<B>) -> ReadTableResult<'_, B::Error, S> {
		let lock = gateway.guard.read();
		let res = Box::pin(async move {
			let table = self.table.unwrap_unchecked();

			let backend = gateway.backend();

			let keys = backend.get_keys::<Vec<_>>(&table).await?;

			let keys_borrowed = keys.iter().map(String::as_str).collect::<Vec<_>>();

			let data = backend.get_all(&table, &keys_borrowed).await?;

			Ok(data)
		});

		RwLockReadGuard::unlock_fair(lock);
		res
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_table()
	}
}

impl<B: Backend, S: Entry + 'static> ActionRunner<B, (), ActionRunError<B::Error>>
	for Action<S, CreateOperation, TableTarget>
{
	unsafe fn run(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<(), ActionRunError<B::Error>>> + Send + '_>> {
		let lock = gateway.guard.write();
		let res = Box::pin(async move {
			let table = self.table.unwrap_unchecked();

			let backend = gateway.backend();

			backend.create_table(&table).await?;

			Ok(())
		});

		RwLockWriteGuard::unlock_fair(lock);
		res
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_table()
	}
}

impl<B: Backend, S: Entry + 'static> ActionRunner<B, bool, ActionRunError<B::Error>>
	for Action<S, DeleteOperation, TableTarget>
{
	unsafe fn run(
		self,
		gateway: &Gateway<B>,
	) -> Pin<Box<dyn Future<Output = Result<bool, ActionRunError<B::Error>>> + Send + '_>> {
		let lock = gateway.guard.write();
		let res = Box::pin(async move {
			let table = self.table.unwrap_unchecked();

			let backend = gateway.backend();

			let exists = backend.has_table(&table).await?;

			backend.delete_table(&table).await?;

			let new_exists = backend.has_table(&table).await?;

			Ok(exists != new_exists)
		});

		RwLockWriteGuard::unlock_fair(lock);
		res
	}

	fn validate(&self) -> Result<(), ActionValidationError> {
		self.validate_table()
	}
}

#[cfg(all(test, feature = "cache"))]
mod tests {
	use serde::{Deserialize, Serialize};

	use super::{
		error::ActionError, Action, ActionKind, ActionValidationError, CreateEntryAction,
		CreateOperation, CreateTableAction, DeleteEntryAction, DeleteOperation, DeleteTableAction,
		EntryTarget, OperationTarget, ReadEntryAction, ReadOperation, ReadTableAction, TableTarget,
		UpdateEntryAction, UpdateOperation,
	};
	use crate::{backend::CacheBackend, error::CacheError, Gateway, IndexEntry};

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

	async fn setup_gateway() -> Gateway<CacheBackend> {
		let backend = CacheBackend::new();
		Gateway::new(backend).await.unwrap()
	}

	async fn setup_table(gateway: &Gateway<CacheBackend>) {
		let action: CreateTableAction<Settings> = Action::new().set_table("table").clone();

		gateway.run(action).await.unwrap().unwrap()
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

		#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
		struct NewSettings;

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
	fn validation_methods() -> Result<(), ActionValidationError> {
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

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn basic_run() -> Result<(), ActionError<CacheError>> {
		let gateway = setup_gateway().await;
		let mut action: Action<Settings, CreateOperation, TableTarget> = Action::new();

		action.set_table("table");

		gateway.run(action).await??;

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
	async fn duplicate_creates() -> Result<(), ActionError<CacheError>> {
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
	async fn read_and_update() -> Result<(), ActionError<CacheError>> {
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
	async fn deletes() -> Result<(), ActionError<CacheError>> {
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

		assert!(gateway.run(read_table).await?.is_err());

		Ok(())
	}
}
