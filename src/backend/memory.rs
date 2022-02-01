use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	iter::FromIterator,
};

use dashmap::DashMap;
use futures_util::{
	future::{err, ok},
	FutureExt,
};
use serde_value::{to_value, DeserializerError, SerializerError, Value};

use super::{
	futures::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetAllFuture, GetFuture,
		GetKeysFuture, HasFuture, HasTableFuture, UpdateFuture,
	},
	Backend,
};
use crate::Entry;

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
}

impl Backend for MemoryBackend {
	type Error = MemoryError;

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		ok(self.tables.contains_key(table)).boxed()
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		self.tables.insert(table.to_owned(), DashMap::new());

		ok(()).boxed()
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		self.tables.remove(table);

		ok(()).boxed()
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		async move {
			self.tables.get(table).map_or_else(
				|| Ok(None.into_iter().collect()),
				|table| Ok(table.clone().into_iter().map(|(key, _)| key).collect()),
			)
		}
		.boxed()
	}

	fn get_all<'a, D, I>(
		&'a self,
		table: &'a str,
		entries: &'a [&str],
	) -> GetAllFuture<'a, I, Self::Error>
	where
		D: Entry,
		I: FromIterator<D>,
	{
		async move {
			self.tables.get(table).map_or_else(
				|| Ok(None.into_iter().collect::<I>()),
				|table| {
					table
						.clone()
						.into_iter()
						.filter_map(|(key, value)| {
							if entries.contains(&key.as_str()) {
								Some(value.deserialize_into().map_err(MemoryError::from))
							} else {
								None
							}
						})
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

	fn create<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> CreateFuture<'a, Self::Error>
	where
		S: Entry,
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

	fn update<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> UpdateFuture<'a, Self::Error>
	where
		S: Entry,
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

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use serde::{Deserialize, Serialize};
	use serde_value::to_value;
	use static_assertions::assert_impl_all;

	use crate::{
		backend::{Backend, MemoryBackend, MemoryError},
		error::MemoryErrorType,
	};

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

		cache_backend.tables.get("test").unwrap().insert(
			"key".to_owned(),
			to_value("value").map_err(|e| MemoryError {
				kind: MemoryErrorType::Serialization,
				source: Some(Box::new(e)),
			})?,
		);

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

		assert!(cache_backend.create("test", "foo", &settings).await.is_ok());

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
