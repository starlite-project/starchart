//! todo

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "json")]
pub use self::json::JsonBackend;

#[cfg(feature = "json")]
#[cfg_attr(feature = "json", doc(hidden))]
pub use self::json::JsonError;

/// The backend to be used with a [`Database`].
///
/// [`Database`]: crate::Database
#[async_trait]
pub trait Backend {
    /// The [`Error`] type that the backend will report up.
    ///
    /// [`Error`]: std::error::Error
    type Error: Send + Sync + StdError;

    /// An optional initialization function, useful for making connections to the database.
    ///
    /// The default impl does nothing
    async fn init(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// An optional shutdown function, useful for disconnecting from databases gracefully.
    ///
    /// The default impl does nothing
    async fn shutdown(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Check if a table exists in the [`Database`].
    ///
    /// [`Database`]: crate::Database
    async fn has_table(&self, table: &str) -> Result<bool, Self::Error>;

    /// Inserts or creates a table in the [`Database`].
    ///
    /// [`Database`]: crate::Database
    async fn create_table(&self, table: &str) -> Result<(), Self::Error>;

    /// Deletes or drops a table from the [`Database`].
    ///
    /// [`Database`]: crate::Database
    async fn delete_table(&self, table: &str) -> Result<(), Self::Error>;

    /// Ensures a table exists in the [`Database`].
    /// Uses [`has_table`] first, then [`create_table`] if it returns false.
    ///
    /// [`Database`]: crate::Database
    /// [`has_table`]: Self::has_table
    /// [`create_table`]: Self::create_table
    async fn ensure_table(&self, table: &str) -> Result<(), Self::Error> {
        if !self.has_table(table).await? {
            self.create_table(table).await?;
        }

        Ok(())
    }

    /// Gets all entries that match a predicate, to get all entries, use [`get_keys`] first.
    ///
    /// [`get_keys`]: Self::get_keys
    async fn get_all<D, I>(&self, table: &str, entries: &[&str]) -> Result<I, Self::Error>
    where
        D: for<'de> Deserialize<'de> + Send + Sync,
        I: FromIterator<D>,
    {
        let keys: Vec<String> = self
            .get_keys::<Vec<_>>(table)
            .await?
            .into_iter()
            .filter(|value| entries.contains(&value.as_str()))
            .collect();

        let mut output = Vec::with_capacity(entries.len());

        for key in keys {
            output.push(self.get(table, key.as_str()).await);
        }

        output.into_iter().collect::<Result<I, Self::Error>>()
    }

    /// Gets all the keys in the table.
    async fn get_keys<I>(&self, table: &str) -> Result<I, Self::Error>
    where
        I: FromIterator<String>;

    /// Gets a certain entry from a table.
    async fn get<D>(&self, table: &str, id: &str) -> Result<D, Self::Error>
    where
        D: for<'de> Deserialize<'de> + Send + Sync;

    /// Checks if an entry exists in a table.
    async fn has(&self, table: &str, id: &str) -> Result<bool, Self::Error>;

    /// Inserts a new entry into a table.
    async fn create<S>(&self, table: &str, id: &str, value: &S) -> Result<(), Self::Error>
    where
        S: Serialize + Send + Sync;

    /// Updates an existing entry in a table.
    async fn update<S>(&self, table: &str, id: &str, value: &S) -> Result<(), Self::Error>
    where
        S: Serialize + Send + Sync;

    /// Replaces an existing entry in a table.
    async fn replace<S>(&self, table: &str, id: &str, value: &S) -> Result<(), Self::Error>
    where
        S: Serialize + Send + Sync;

    /// Deletes an entry from a table.
    async fn delete(&self, table: &str, id: &str) -> Result<(), Self::Error>;
}
