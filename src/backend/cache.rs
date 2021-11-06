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
