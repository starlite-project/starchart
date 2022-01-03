use std::iter::FromIterator;

use dashmap::{mapref::one::Ref, DashMap};
use serde_value::{to_value, DeserializerError, SerializerError, Value};
use thiserror::Error;

use super::{
	future::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
		HasFuture, HasTableFuture, ReplaceFuture, UpdateFuture,
	},
	Backend,
};
use crate::Entry;

/// An error returned from the [`MemoryBackend`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MemoryError {
	/// A serialization error occurred.
	#[error("a serialization error occurred")]
	Serialization(#[from] SerializerError),
	/// A deserialization error occurred.
	#[error("a deserialization error occurred")]
	DeserializationError(#[from] DeserializerError),
	/// The specified table doesn't exist
	#[error("the table {0} does not exist")]
	TableDoesntExist(String),
	/// The value already exists
	#[error("value already exists")]
	ValueAlreadyExists,
}

/// A memory-based backend, uses a [`DashMap`] of [`Value`]s
/// to represent data.
#[derive(Debug, Default, Clone)]
#[cfg(feature = "memory")]
pub struct MemoryBackend {
	tables: DashMap<String, DashMap<String, Value>>,
}

impl MemoryBackend {
	/// Creates a new [`MemoryBackend`].
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	fn get_table<'a>(
		&'a self,
		table: &'a str,
	) -> Result<Ref<'a, String, DashMap<String, Value>>, MemoryError> {
		match self.tables.get(table) {
			Some(table) => Ok(table),
			None => Err(MemoryError::TableDoesntExist(table.to_owned())),
		}
	}
}

impl Backend for MemoryBackend {
	type Error = MemoryError;

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		Box::pin(async move { Ok(self.tables.contains_key(table)) })
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		Box::pin(async move {
			self.tables.insert(table.to_owned(), DashMap::new());

			Ok(())
		})
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		Box::pin(async move {
			self.tables.remove(table);

			Ok(())
		})
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		Box::pin(async move {
			let table_value = self.get_table(table)?;

			let keys = table_value.clone().into_iter().map(|(key, _)| key);

			Ok(keys.collect())
		})
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		Box::pin(async move {
			let table_value = self.get_table(table)?;

			let value = match table_value.get(id) {
				None => return Ok(None),
				Some(json) => json.value().clone(),
			};

			Ok(value.deserialize_into()?)
		})
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		Box::pin(async move {
			let table_value = self.get_table(table)?;

			Ok(table_value.value().contains_key(id))
		})
	}

	fn create<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> CreateFuture<'a, Self::Error>
	where
		S: Entry,
	{
		Box::pin(async move {
			let table_value = self.get_table(table)?;

			if table_value.contains_key(id) {
				return Err(MemoryError::ValueAlreadyExists);
			}

			let serialized = to_value(value)?;

			table_value.insert(id.to_owned(), serialized);

			Ok(())
		})
	}

	fn update<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> UpdateFuture<'a, Self::Error>
	where
		S: Entry,
	{
		Box::pin(async move {
			let table_value = self.get_table(table)?;

			table_value.insert(id.to_owned(), to_value(value)?);

			Ok(())
		})
	}

	fn replace<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> ReplaceFuture<'a, Self::Error>
	where
		S: Entry,
	{
		Box::pin(async move {
			self.update(table, id, value).await?;

			Ok(())
		})
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
		Box::pin(async move {
			let table_value = self.get_table(table)?;

			table_value.remove(id);

			Ok(())
		})
	}
}

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use dashmap::DashMap;
	use serde::{Deserialize, Serialize};
	use serde_value::to_value;
	use static_assertions::assert_impl_all;

	use crate::backend::{Backend, MemoryBackend, MemoryError};

	assert_impl_all!(MemoryBackend: Backend, Clone, Debug, Default, Send, Sync);

	#[derive(
		Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
	)]
	struct Settings {
		option: bool,
		times: u32,
	}

	#[test]
	fn new() {
		let cache_backend = MemoryBackend::new();

		assert_eq!(cache_backend.tables.len(), 0);
	}

	#[test]
	fn get_table() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		cache_backend
			.tables
			.insert("test".to_owned(), DashMap::new());

		let table = cache_backend.get_table("test")?;

		assert_eq!(table.key(), "test");

		assert!(cache_backend.get_table("test2").is_err());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_table() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		assert!(!cache_backend.has_table("test").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn create_table() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		cache_backend.create_table("test").await?;

		assert!(cache_backend.tables.contains_key("test"));

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn delete_table() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		cache_backend.create_table("test").await?;

		assert!(cache_backend.tables.contains_key("test"));

		cache_backend.delete_table("test").await?;

		assert!(!cache_backend.tables.contains_key("test"));

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		cache_backend.create_table("test").await?;

		cache_backend
			.tables
			.get("test")
			.unwrap()
			.insert("key".to_owned(), to_value("value")?);

		let keys = cache_backend.get_keys::<Vec<_>>("test").await?;

		assert_eq!(keys, vec!["key".to_owned()]);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_and_create() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		let settings = Settings {
			option: true,
			times: 42,
		};

		cache_backend.create_table("test").await?;

		cache_backend.create("test", "foo", &settings).await?;

		let settings = cache_backend.get::<Settings>("test", "foo").await?;

		assert_eq!(
			settings,
			Some(Settings {
				option: true,
				times: 42
			})
		);

		let not_existing = cache_backend.get::<Settings>("test", "bar").await?;

		assert_eq!(not_existing, None);

		assert!(cache_backend
			.create("test", "foo", &settings)
			.await
			.is_err());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		cache_backend.create_table("test").await?;

		cache_backend
			.create(
				"test",
				"foo",
				&Settings {
					option: true,
					times: 42,
				},
			)
			.await?;

		assert!(cache_backend.has("test", "foo").await?);

		assert!(!cache_backend.has("test", "bar").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn update_and_replace() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		cache_backend.create_table("test").await?;

		cache_backend
			.create(
				"test",
				"foo",
				&Settings {
					option: true,
					times: 42,
				},
			)
			.await?;

		cache_backend
			.update(
				"test",
				"foo",
				&Settings {
					option: false,
					times: 43,
				},
			)
			.await?;

		assert_eq!(
			cache_backend.get::<Settings>("test", "foo").await?,
			Some(Settings {
				option: false,
				times: 43
			})
		);

		cache_backend
			.replace(
				"test",
				"foo",
				&Settings {
					option: true,
					times: 44,
				},
			)
			.await?;

		assert_eq!(
			cache_backend.get::<Settings>("test", "foo").await?,
			Some(Settings {
				option: true,
				times: 44
			})
		);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn delete() -> Result<(), MemoryError> {
		let cache_backend = MemoryBackend::new();

		cache_backend.create_table("test").await?;

		cache_backend
			.create(
				"test",
				"foo",
				&Settings {
					option: true,
					times: 42,
				},
			)
			.await?;

		assert!(cache_backend.has("test", "foo").await?);

		cache_backend.delete("test", "foo").await?;

		assert!(!cache_backend.has("test", "foo").await?);

		Ok(())
	}
}
