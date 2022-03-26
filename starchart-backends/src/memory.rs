//! A memory based backend. Useful for debugging or applications
//! who only need to store data at runtime.

use std::{
	collections::hash_map::RandomState,
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	hash::BuildHasher,
	iter::FromIterator,
};

use dashmap::DashMap;
use futures_util::{
	future::{err, ok},
	FutureExt,
};
use serde_value::{to_value, DeserializerError, SerializerError, Value};
use starchart::{
	backend::{
		futures::{
			CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetAllFuture,
			GetFuture, HasFuture, HasTableFuture, UpdateFuture,
		},
		Backend,
	},
	Entry,
};

/// An error returned from the [`MemoryBackend`].
#[cfg(feature = "memory")]
#[derive(Debug)]
pub struct MemoryError {
	source: Option<Box<dyn Error + Send + Sync>>,
	kind: MemoryErrorType,
}

impl MemoryError {
	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &MemoryErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(self) -> (MemoryErrorType, Option<Box<dyn Error + Send + Sync>>) {
		(self.kind, self.source)
	}
}

impl Display for MemoryError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			MemoryErrorType::Serialization => f.write_str("a serialization error occurred"),
			MemoryErrorType::Deserialization => f.write_str("a deserialization error occurred"),
		}
	}
}

impl Error for MemoryError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn Error + 'static))
	}
}

impl From<SerializerError> for MemoryError {
	fn from(err: SerializerError) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: MemoryErrorType::Serialization,
		}
	}
}

impl From<DeserializerError> for MemoryError {
	fn from(err: DeserializerError) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: MemoryErrorType::Deserialization,
		}
	}
}

impl From<MemoryError> for starchart::Error {
	fn from(e: MemoryError) -> Self {
		Self::backend(Box::new(e))
	}
}

/// The type of [`MemoryError`] that occurred.
#[cfg(feature = "memory")]
#[allow(missing_copy_implementations)]
#[derive(Debug)]
#[non_exhaustive]
pub enum MemoryErrorType {
	/// A serialization error occurred.
	Serialization,
	/// A deserialization error occurred.
	Deserialization,
}

/// A memory-based backend, uses a [`DashMap`] of [`Value`]s
/// to represent data.
#[cfg(feature = "memory")]
#[must_use = "a memory backend does nothing on it's own"]
#[repr(transparent)]
pub struct MemoryBackend<S = RandomState> {
	tables: DashMap<String, DashMap<String, Value, S>, S>,
}

impl MemoryBackend<RandomState> {
	/// Creates a new [`MemoryBackend`].
	pub fn new() -> Self {
		Self::with_capacity_and_hasher(0, RandomState::default())
	}

	/// Creates a new [`MemoryBackend`] with the specified capacity.
	pub fn with_capacity(cap: usize) -> Self {
		Self::with_capacity_and_hasher(cap, RandomState::default())
	}
}

impl<S: BuildHasher + Clone> MemoryBackend<S> {
	/// Creates a new [`MemoryBackend`] with the specified hasher.
	pub fn with_hasher(hasher: S) -> Self {
		Self::with_capacity_and_hasher(0, hasher)
	}

	/// Creates a new [`MemoryBackend`] with the specified capacity and hasher.
	pub fn with_capacity_and_hasher(cap: usize, hasher: S) -> Self {
		Self {
			tables: DashMap::with_capacity_and_hasher(cap, hasher),
		}
	}
}

impl<S: BuildHasher + Clone> Debug for MemoryBackend<S> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("MemoryBackend")
			.field("tables", &self.tables)
			.finish()
	}
}

impl<S: Default + BuildHasher + Clone> Default for MemoryBackend<S> {
	fn default() -> Self {
		Self {
			tables: DashMap::default(),
		}
	}
}

impl<S: Clone> Clone for MemoryBackend<S> {
	fn clone(&self) -> Self {
		Self {
			tables: self.tables.clone(),
		}
	}
}

impl<S: BuildHasher + Clone + Send + Sync> Backend for MemoryBackend<S> {
	type Error = MemoryError;

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		ok(self.tables.contains_key(table)).boxed()
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		self.tables.insert(
			table.to_owned(),
			DashMap::with_hasher(self.tables.hasher().clone()),
		);

		ok(()).boxed()
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		self.tables.remove(table);

		ok(()).boxed()
	}

	fn get_all<'a, D, I>(&'a self, table: &'a str) -> GetAllFuture<'a, I, Self::Error>
	where
		D: Entry,
		I: FromIterator<(String, D)>,
	{
		async move {
			self.tables.get(table).map_or_else(
				|| Ok(None.into_iter().collect::<I>()),
				|table| {
					table
						.clone()
						.into_iter()
						.map(|(k, v)| Ok((k, v.deserialize_into().map_err(MemoryError::from)?)))
						.collect::<Result<I, Self::Error>>()
				},
			)
		}
		.boxed()
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		async move {
			if let Some(table) = self.tables.get(table) {
				let value = match table.get(id) {
					None => return Ok(None),
					Some(json) => json.value().clone(),
				};

				Ok(Some(value.deserialize_into()?))
			} else {
				Ok(None)
			}
		}
		.boxed()
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		ok(self
			.tables
			.get(table)
			.map_or(false, |table| table.contains_key(id)))
		.boxed()
	}

	fn create<'a, E>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a E,
	) -> CreateFuture<'a, Self::Error>
	where
		E: Entry,
	{
		if let Some(table) = self.tables.get(table) {
			let serialized = match to_value(value) {
				Ok(v) => v,
				Err(e) => return err(e.into()).boxed(),
			};

			table.insert(id.to_owned(), serialized);
		}

		ok(()).boxed()
	}

	fn update<'a, E>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a E,
	) -> UpdateFuture<'a, Self::Error>
	where
		E: Entry,
	{
		if let Some(table) = self.tables.get(table) {
			let to_replace = match to_value(value) {
				Ok(v) => v,
				Err(e) => return err(e.into()).boxed(),
			};
			table.insert(id.to_owned(), to_replace);
		}

		ok(()).boxed()
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
		if let Some(table) = self.tables.get(table) {
			table.remove(id);
		}

		ok(()).boxed()
	}
}

#[cfg(all(test, not(miri)))]
mod tests {
	use std::{collections::HashMap, fmt::Debug};

	use fxhash::FxBuildHasher;
	use starchart::backend::Backend;
	use static_assertions::assert_impl_all;

	use super::{MemoryBackend, MemoryError};
	use crate::testing::TestSettings;

	assert_impl_all!(MemoryBackend: Backend, Clone, Debug, Default, Send, Sync);

	#[tokio::test]
	async fn table_methods() -> Result<(), MemoryError> {
		let backend = MemoryBackend::with_hasher(FxBuildHasher::default());

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		backend.delete_table("table").await?;

		assert!(!backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	async fn get_and_create() -> Result<(), MemoryError> {
		let backend = MemoryBackend::with_capacity_and_hasher(1, FxBuildHasher::default());

		backend.init().await?;

		backend.create_table("table").await?;
		backend
			.create("table", "1", &TestSettings::default())
			.await?;

		assert_eq!(
			backend.get::<TestSettings>("table", "1").await?,
			Some(TestSettings::default())
		);

		assert_eq!(backend.get::<TestSettings>("table", "2").await?, None);

		let settings = TestSettings {
			id: 2,
			..TestSettings::default()
		};

		assert!(backend.create("table", "2", &settings).await.is_ok());

		assert_eq!(backend.get::<TestSettings>("non_exist", "1").await?, None);

		Ok(())
	}

	#[tokio::test]
	async fn update_and_delete() -> Result<(), MemoryError> {
		let backend = MemoryBackend::with_capacity_and_hasher(1, FxBuildHasher::default());
		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = TestSettings::default();
		backend.create("table", "1", &settings).await?;

		settings.opt = None;

		backend.update("table", "1", &settings).await?;

		assert_eq!(
			backend.get::<TestSettings>("table", "1").await?,
			Some(settings)
		);

		backend.delete("table", "1").await?;

		assert_eq!(backend.get::<TestSettings>("table", "1").await?, None);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_all() -> Result<(), MemoryError> {
		let backend = MemoryBackend::with_capacity(1);
		backend.init().await?;

		backend.ensure_table("table").await?;

		let mut settings = TestSettings::default();
		backend.create("table", "1", &settings).await?;

		settings.opt = None;
		settings.id += 1;

		backend.create("table", "2", &settings).await?;

		settings.value = "goodbye!".to_owned();
		settings.array.extend(&[7, 8, 9]);
		settings.opt = TestSettings::default().opt;
		settings.id += 1;

		backend.create("table", "3", &settings);

		let values: HashMap<_, TestSettings> = backend.get_all("table").await?;

		let mut values = values.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

		values.sort_by(|a, b| a.id.cmp(&b.id));

		assert_eq!(
			values,
			vec![
				TestSettings::default(),
				TestSettings {
					id: 2,
					opt: None,
					..TestSettings::default()
				},
				TestSettings {
					id: 3,
					value: "goodbye!".to_owned(),
					array: vec![1, 2, 3, 4, 5, 7, 8, 9],
					..TestSettings::default()
				}
			]
		);

		assert_eq!(
			backend
				.get_all::<TestSettings, HashMap<_, _>>("non_exist")
				.await?,
			HashMap::new(),
		);

		Ok(())
	}
}
