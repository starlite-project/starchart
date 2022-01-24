//! The backend that fetches and provides data for the [`Starchart`].
//!
//! [`Starchart`]: crate::Starchart

use std::{error::Error as StdError, iter::FromIterator};

use futures_util::{future::join_all, FutureExt};

use self::futures::{
	CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, EnsureFuture,
	EnsureTableFuture, GetAllFuture, GetFuture, GetKeysFuture, HasFuture, HasTableFuture,
	InitFuture, ReplaceFuture, ShutdownFuture, UpdateFuture,
};
use crate::{util::InnerUnwrap, Entry};

pub mod futures;

#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "memory")]
mod memory;
#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "bincode")]
pub use self::bincode::BincodeBackend;
#[cfg(feature = "fs")]
#[doc(hidden)]
pub use self::fs::{FsError, FsErrorType};
#[cfg(feature = "json")]
pub use self::json::JsonBackend;
#[cfg(all(feature = "json", feature = "pretty"))]
pub use self::json::JsonPrettyBackend;
#[cfg(feature = "memory")]
pub use self::memory::MemoryBackend;
#[cfg(feature = "memory")]
#[doc(hidden)]
pub use self::memory::{MemoryError, MemoryErrorType};
#[cfg(feature = "toml")]
pub use self::toml::TomlBackend;
#[cfg(all(feature = "toml", feature = "pretty"))]
pub use self::toml::TomlPrettyBackend;
#[cfg(feature = "yaml")]
pub use self::yaml::YamlBackend;

/// The backend to be used to manage data.
pub trait Backend: Send + Sync {
	/// The [`Error`] type that the backend will report up.
	///
	/// [`Error`]: std::error::Error
	type Error: Send + Sync + StdError + 'static;

	/// An optional initialization function, useful for making connections to the database.
	///
	/// The default impl does nothing
	fn init(&self) -> InitFuture<'_, Self::Error> {
		Box::pin(async { Ok(()) })
	}

	/// An optional shutdown function, useful for disconnecting from databases gracefully.
	///
	/// The default impl does nothing
	///
	/// # Safety
	///
	/// This should not fail, as it's ran upon dropping the [`Starchart`],
	/// and panicking during a drop means resources haven't adequately been cleaned up,
	/// which isn't inherintly UB however it should still be documented.
	///
	/// [`Starchart`]: crate::Starchart
	unsafe fn shutdown(&self) -> ShutdownFuture {
		Box::pin(async {})
	}

	/// Check if a table exists.
	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error>;

	/// Inserts or creates a table.
	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error>;

	/// Deletes or drops a table.
	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error>;

	/// Ensures a table exists.
	/// Uses [`Self::has_table`] first, then [`Self::create_table`] if it returns false.
	fn ensure_table<'a>(&'a self, table: &'a str) -> EnsureTableFuture<'a, Self::Error> {
		async move {
			if !self.has_table(table).await? {
				self.create_table(table).await?;
			}

			Ok(())
		}
		.boxed()
	}

	/// Gets all entries that match a predicate, to get all entries, use [`get_keys`] first.
	///
	/// [`get_keys`]: Self::get_keys
	fn get_all<'a, D, I>(
		&'a self,
		table: &'a str,
		entries: &'a [&'a str],
	) -> GetAllFuture<'a, I, Self::Error>
	where
		D: Entry,
		I: FromIterator<D>,
	{
		async move {
			let gets = entries.iter().copied().map(|v| self.get::<D>(table, v));

			join_all(gets)
				.await
				.into_iter()
				.filter_map(Result::transpose)
				.collect::<Result<I, Self::Error>>()
		}
		.boxed()
	}

	/// Gets all the keys in the table.
	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>;

	/// Gets a certain entry from a table.
	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry;

	/// Checks if an entry exists in a table.
	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error>;

	/// Inserts a new entry into a table.
	fn create<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> CreateFuture<'a, Self::Error>
	where
		S: Entry;

	/// Ensures a value exists in the table.
	fn ensure<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> EnsureFuture<'a, Self::Error>
	where
		S: Entry,
	{
		async move {
			if !self.has(table, id).await? {
				self.create(table, id, value).await?;
			}

			Ok(())
		}
		.boxed()
	}

	/// Updates an existing entry in a table.
	fn update<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> UpdateFuture<'a, Self::Error>
	where
		S: Entry;

	/// Replaces an existing entry in a table.
	fn replace<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> ReplaceFuture<'a, Self::Error>
	where
		S: Entry;

	/// Deletes an entry from a table.
	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error>;
}

// need to cfg for cache because otherwise we would get errors if someone just ran `cargo test`
#[cfg(all(test, feature = "memory"))]
mod tests {

	use super::{Backend, MemoryBackend, MemoryError};

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() {
		let backend = MemoryBackend::new();

		assert!(Backend::init(&backend).await.is_ok());
	}

	// The default impl does nothing, this is just for coverage
	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn shutdown() {
		let backend = MemoryBackend::new();

		unsafe {
			Backend::shutdown(&backend).await;
		}
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn ensure_table() -> Result<(), MemoryError> {
		let backend = MemoryBackend::new();

		let table_name = "test";
		assert!(!backend.has_table(table_name).await?);

		Backend::ensure_table(&backend, table_name).await?;

		assert!(backend.has_table(table_name).await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn ensure() -> Result<(), MemoryError> {
		let backend = MemoryBackend::new();

		let table_name = "test";
		let id = "id";
		let value = "value".to_owned();

		backend.create_table(table_name).await?;

		assert!(!backend.has(table_name, id).await?);

		Backend::ensure(&backend, table_name, id, &value).await?;

		assert!(backend.has(table_name, id).await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_all() -> Result<(), MemoryError> {
		let backend = MemoryBackend::new();

		backend.create_table("test").await?;

		backend.create("test", "id", &"value".to_owned()).await?;
		backend.create("test", "id2", &"value2".to_owned()).await?;

		let keys = ["id", "id2", "doesn't exist"];
		let mut values: Vec<String> = backend.get_all("test", &keys).await?;
		let mut expected = vec!["value".to_owned(), "value2".to_owned()];

		values.sort();
		expected.sort();

		assert_eq!(values, expected);

		Ok(())
	}
}
