//! todo

use self::future::{
    CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, EnsureFuture,
    EnsureTableFuture, GetAllFuture, GetFuture, GetKeysFuture, HasFuture, HasTableFuture,
    InitFuture, ReplaceFuture, ShutdownFuture, UpdateFuture,
};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

pub mod future;

mod cache;
#[cfg(feature = "json")]
mod json;

pub use self::cache::CacheBackend;
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
        D: for<'de> Deserialize<'de> + Send + Sync,
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
        D: for<'de> Deserialize<'de> + Send + Sync;

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
        S: Serialize + Send + Sync;

    /// Ensures a value exists in the table.
    fn ensure<'a, S>(
        &'a self,
        table: &'a str,
        id: &'a str,
        value: &'a S,
    ) -> EnsureFuture<'a, Self::Error>
    where
        S: Serialize + Send + Sync,
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
        S: Serialize + Send + Sync;

    /// Replaces an existing entry in a table.
    fn replace<'a, S>(
        &'a self,
        table: &'a str,
        id: &'a str,
        value: &'a S,
    ) -> ReplaceFuture<'a, Self::Error>
    where
        S: Serialize + Send + Sync;

    /// Deletes an entry from a table.
    fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error>;
}
