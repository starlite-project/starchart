use super::{
	future::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
		HasFuture, HasTableFuture, ReplaceFuture, UpdateFuture,
	},
	Backend,
};
use dashmap::{mapref::one::Ref, DashMap};
use serde::{Deserialize, Serialize};
use serde_value::{to_value, DeserializerError, SerializerError, Value};
use std::iter::FromIterator;
use thiserror::Error;

/// An error returned from the [`CacheBackend`].
#[doc(cfg(feature = "cache"))]
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CacheError {
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
#[doc(cfg(feature = "cache"))]
#[derive(Debug, Default, Clone)]
pub struct CacheBackend {
	tables: DashMap<String, DashMap<String, Value>>,
}

impl CacheBackend {
	/// Creates a new [`CacheBackend`].
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	fn get_table<'a>(
		&'a self,
		table: &'a str,
	) -> Result<Ref<'a, String, DashMap<String, Value>>, CacheError> {
		match self.tables.get(table) {
			Some(table) => Ok(table),
			None => Err(CacheError::TableDoesntExist(table.to_owned())),
		}
	}
}

impl Backend for CacheBackend {
	type Error = CacheError;

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
		D: for<'de> Deserialize<'de> + Send + Sync,
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
		S: Serialize + Send + Sync,
	{
		Box::pin(async move {
			let table_value = self.get_table(table)?;

			if table_value.contains_key(id) {
				return Err(CacheError::ValueAlreadyExists);
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
		S: Serialize + Send + Sync,
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
		S: Serialize + Send + Sync,
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
	use super::{CacheBackend, CacheError};
	use crate::{backend::Backend, test_utils::SyncFuture};
	use dashmap::DashMap;
	use serde::{Deserialize, Serialize};
	use serde_value::to_value;
	use static_assertions::assert_impl_all;
	use std::fmt::Debug;

	assert_impl_all!(CacheBackend: Clone, Debug, Default, crate::backend::Backend);

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
	struct Settings {
		option: bool,
		times: u32,
	}

	#[test]
	fn new() {
		let cache_backend = CacheBackend::new();

		assert_eq!(cache_backend.tables.len(), 0);
	}

	#[test]
	fn get_table() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		cache_backend
			.tables
			.insert("test".to_owned(), DashMap::new());

		let table = cache_backend.get_table("test")?;

		assert_eq!(table.key(), "test");

		assert!(cache_backend.get_table("test2").is_err());

		Ok(())
	}

	#[test]
	fn has_table() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		assert!(!cache_backend.has_table("test").wait()?);

		Ok(())
	}

	#[test]
	fn create_table() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		cache_backend.create_table("test").wait()?;

		assert!(cache_backend.tables.contains_key("test"));

		Ok(())
	}

	#[test]
	fn delete_table() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		cache_backend.create_table("test").wait()?;

		assert!(cache_backend.tables.contains_key("test"));

		cache_backend.delete_table("test").wait()?;

		assert!(!cache_backend.tables.contains_key("test"));

		Ok(())
	}

	#[test]
	fn get_keys() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		cache_backend.create_table("test").wait()?;

		cache_backend
			.tables
			.get("test")
			.unwrap()
			.insert("key".to_owned(), to_value("value")?);

		let keys = cache_backend.get_keys::<Vec<_>>("test").wait()?;

		assert_eq!(keys, vec!["key".to_owned()]);

		Ok(())
	}

	#[test]
	fn get_and_create() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		let settings = Settings {
			option: true,
			times: 42,
		};

		cache_backend.create_table("test").wait()?;

		cache_backend.create("test", "foo", &settings).wait()?;

		let settings = cache_backend.get::<Settings>("test", "foo").wait()?;

		assert_eq!(
			settings,
			Some(Settings {
				option: true,
				times: 42
			})
		);

		let not_existing = cache_backend.get::<Settings>("test", "bar").wait()?;

		assert_eq!(not_existing, None);

		assert!(cache_backend
			.create("test", "foo", &settings)
			.wait()
			.is_err());

		Ok(())
	}

	#[test]
	fn has() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		cache_backend.create_table("test").wait()?;

		cache_backend
			.create(
				"test",
				"foo",
				&Settings {
					option: true,
					times: 42,
				},
			)
			.wait()?;

		assert!(cache_backend.has("test", "foo").wait()?);

		assert!(!cache_backend.has("test", "bar").wait()?);

		Ok(())
	}

	#[test]
	fn update_and_replace() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		cache_backend.create_table("test").wait()?;

		cache_backend
			.create(
				"test",
				"foo",
				&Settings {
					option: true,
					times: 42,
				},
			)
			.wait()?;

		cache_backend
			.update(
				"test",
				"foo",
				&Settings {
					option: false,
					times: 43,
				},
			)
			.wait()?;

		assert_eq!(
			cache_backend.get::<Settings>("test", "foo").wait()?,
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
			.wait()?;

		assert_eq!(
			cache_backend.get::<Settings>("test", "foo").wait()?,
			Some(Settings {
				option: true,
				times: 44
			})
		);

		Ok(())
	}

	#[test]
	fn delete() -> Result<(), CacheError> {
		let cache_backend = CacheBackend::new();

		cache_backend.create_table("test").wait()?;

		cache_backend
			.create(
				"test",
				"foo",
				&Settings {
					option: true,
					times: 42,
				},
			)
			.wait()?;

		assert!(cache_backend.has("test", "foo").wait()?);

		cache_backend.delete("test", "foo").wait()?;

		assert!(!cache_backend.has("test", "foo").wait()?);

		Ok(())
	}
}
