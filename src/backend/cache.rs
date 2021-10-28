use super::{
    future::{
        CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
        HasFuture, HasTableFuture, ReplaceFuture, UpdateFuture,
    },
    Backend,
};
use dashmap::{mapref::one::Ref, DashMap};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("a JSON error occurred")]
    SerdeJson(#[from] serde_json::Error),
    #[error("the table {0} does not exist")]
    TableDoesntExist(String),
    #[error("value already exists")]
    ValueAlreadyExists,
}

/// todo
#[derive(Debug, Default, Clone)]
pub struct CacheBackend {
    tables: DashMap<String, DashMap<String, String>>,
}

impl CacheBackend {
    /// todo
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    unsafe fn get_table<'a>(
        &'a self,
        table: &'a str,
    ) -> Result<Ref<'a, String, DashMap<String, String>>, CacheError> {
        let temp = self.tables.get(table);

        if temp.is_none() {
            return Err(CacheError::TableDoesntExist(table.to_owned()));
        }

        Ok(temp.unwrap_unchecked())
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
            let table_value = unsafe { self.get_table(table)? };

            let keys = table_value.clone().into_iter().map(|(key, _)| key);

            Ok(keys.collect())
        })
    }

    fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
    where
        D: for<'de> Deserialize<'de> + Send + Sync,
    {
        Box::pin(async move {
            let table_value = unsafe { self.get_table(table)? };

            let json = unsafe {
                let entry = table_value.get(id);

                if entry.is_none() {
                    return Ok(None);
                }

                entry.unwrap_unchecked()
            };

            Ok(Some(from_str(json.value().as_str())?))
        })
    }

    fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
        Box::pin(async move {
            let table_value = unsafe { self.get_table(table)? };

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
            let table_value = unsafe { self.get_table(table)? };

            let exists = table_value.get(id);

            if exists.is_some() {
                return Err(CacheError::ValueAlreadyExists);
            }

            let serialized = to_string(value)?;

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
            let table_value = unsafe { self.get_table(table)? };

            table_value.insert(id.to_owned(), to_string(value)?);

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
            let table_value = unsafe { self.get_table(table)? };

            table_value.remove(id);

            Ok(())
        })
    }
}
