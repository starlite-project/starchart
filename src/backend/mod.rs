//! The backend that fetches and provides data for a [`Database`].
//!
//! [`Database`]: crate::database::Database

use std::{error::Error as StdError, iter::FromIterator};

use self::future::{
	CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, EnsureFuture,
	EnsureTableFuture, GetAllFuture, GetFuture, GetKeysFuture, HasFuture, HasTableFuture,
	InitFuture, ReplaceFuture, ShutdownFuture, UpdateFuture,
};
use crate::Entry;

pub mod future;

#[cfg(feature = "cache")]
mod cache;
#[cfg(feature = "json")]
mod json;

#[cfg(feature = "cache")]
pub use self::cache::CacheBackend;
#[cfg(feature = "cache")]
#[cfg_attr(feature = "cache", doc(hidden))]
pub use self::cache::CacheError;
#[cfg(feature = "json")]
pub use self::json::JsonBackend;
#[cfg(feature = "json")]
#[cfg_attr(feature = "json", doc(hidden))]
pub use self::json::JsonError;

/// The backend to be used with a [`Database`].
///
/// [`Database`]: crate::Database
pub trait Backend: Send + Sync {
	/// The [`Error`] type that the backend will report up.
	///
	/// [`Error`]: std::error::Error
	type Error: Send + Sync + StdError;

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
	/// This should not fail, as it's ran upon dropping the [`Gateway`],
	/// and panicking during a drop means resources haven't adequately been cleaned up,
	/// which isn't inherintly UB however it should still be documented.
	///
	/// [`Gateway`]: crate::Gateway
	unsafe fn shutdown(&self) -> ShutdownFuture {
		Box::pin(async {})
	}

	/// Check if a table exists in the [`Database`].
	///
	/// [`Database`]: crate::Database
	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error>;

	/// Inserts or creates a table in the [`Database`].
	///
	/// [`Database`]: crate::Database
	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error>;

	/// Deletes or drops a table from the [`Database`].
	///
	/// [`Database`]: crate::Database
	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error>;

	/// Ensures a table exists in the [`Database`].
	/// Uses [`has_table`] first, then [`create_table`] if it returns false.
	///
	/// [`Database`]: crate::Database
	/// [`has_table`]: Self::has_table
	/// [`create_table`]: Self::create_table
	fn ensure_table<'a>(&'a self, table: &'a str) -> EnsureTableFuture<'a, Self::Error> {
		Box::pin(async move {
			if !self.has_table(table).await? {
				self.create_table(table).await?;
			}

			Ok(())
		})
	}

	/// Gets all entries that match a predicate, to get all entries, use [`get_keys`] first.
	///
	/// [`get_keys`]: Self::get_keys
	fn get_all<'a, D, I>(
		&'a self,
		table: &'a str,
		entries: &'a [&str],
	) -> GetAllFuture<'a, I, Self::Error>
	where
		D: Entry,
		I: FromIterator<D>,
	{
		Box::pin(async move {
			let mut output = Vec::with_capacity(entries.len());

			for key in entries.iter().copied() {
				let value = self.get(table, key).await?;
				if value.is_none() {
					continue;
				}
				output.push(unsafe { value.unwrap_unchecked() });
			}

			Ok(output.into_iter().collect())
		})
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
		Box::pin(async move {
			if !self.has(table, id).await? {
				self.create(table, id, value).await?;
			}

			Ok(())
		})
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
#[cfg(all(test, feature = "cache"))]
mod tests {

	use super::{Backend, CacheBackend, CacheError};

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() {
		let backend = CacheBackend::new();

		assert!(Backend::init(&backend).await.is_ok());
	}

	// The default impl does nothing, this is just for coverage
	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn shutdown() {
		let backend = CacheBackend::new();

		unsafe {
			Backend::shutdown(&backend).await;
		}
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn ensure_table() -> Result<(), CacheError> {
		let backend = CacheBackend::new();

		let table_name = "test";
		assert!(!backend.has_table(table_name).await?);

		Backend::ensure_table(&backend, table_name).await?;

		assert!(backend.has_table(table_name).await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn ensure() -> Result<(), CacheError> {
		let backend = CacheBackend::new();

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
	async fn get_all() -> Result<(), CacheError> {
		let backend = CacheBackend::new();

		backend.create_table("test").await?;

		backend.create("test", "id", &"value".to_owned()).await?;
		backend.create("test", "id2", &"value2".to_owned()).await?;

		let keys = vec!["id", "id2", "id3"];
		let values: Vec<String> = backend.get_all("test", &keys).await?;

		assert_eq!(values, vec!["value".to_owned(), "value2".to_owned()]);

		Ok(())
	}
}
