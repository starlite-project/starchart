//! todo

use crate::{backend::Backend, database::DatabaseError, Database};
use dashmap::{mapref::one::Ref, DashMap};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    sync::Arc,
};

/// todo
#[must_use]
pub struct Storage<'a, B>
where
    B: Backend,
{
    inner: Ref<'a, String, Database<B>>,
}

impl<'a, B> Storage<'a, B>
where
    B: Backend,
{
    fn new(inner: Ref<'a, String, Database<B>>) -> Self {
        Self { inner }
    }

    /// todo
    #[must_use]
    pub fn key(&'a self) -> &'a String {
        self.inner.key()
    }

    /// todo
    #[must_use]
    pub fn value(&'a self) -> &'a Database<B> {
        self.inner.value()
    }
}

impl<B> Debug for Storage<'_, B>
where
    B: Backend + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Storage")
            .field("inner", self.value())
            .finish()
    }
}

impl<'a, B> Deref for Storage<'a, B>
where
    B: Backend,
{
    type Target = Database<B>;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

/// todo
#[derive(Debug, Clone)]
pub struct Gateway<B: Backend> {
    backend: Arc<B>,
    databases: DashMap<String, Database<B>>,
}

impl<B: Backend> Gateway<B> {
    /// todo
    ///
    /// # Errors
    ///
    /// todo
    pub async fn new(backend: B) -> Result<Self, B::Error> {
        backend.init().await?;
        Ok(Self {
            backend: Arc::new(backend),
            databases: DashMap::new(),
        })
    }

    /// todo
    /// 
    /// # Errors
    /// 
    /// todo
    pub async fn acquire<'a, S>(
        &'a self,
        table_name: String,
    ) -> Result<Storage<'a, B>, DatabaseError<B::Error>>
    where
        S: Debug + Serialize + for<'de> Deserialize<'de> + 'static,
    {
        let exists = self.get::<S>(&table_name)?;

        if exists.is_some() {
            return Ok(unsafe { exists.unwrap_unchecked() });
        }

        self.create::<S>(table_name).await
    }

    /// todo
    ///
    /// # Errors
    ///
    /// todo
    pub async fn create<'a, S>(
        &'a self,
        table_name: String,
    ) -> Result<Storage<'a, B>, DatabaseError<B::Error>>
    where
        S: Debug + Serialize + for<'de> Deserialize<'de> + 'static,
        B: 'a,
    {
        let mut database = Database::new(table_name.clone(), Arc::clone(&self.backend));

        database.setup::<S>().await?;

        self.databases.insert(table_name.clone(), database);

        Ok(unsafe { self.get_unchecked::<S>(&table_name) })
    }

    /// todo
    ///
    /// # Errors
    ///
    /// todo
    pub fn get<'a, S>(
        &'a self,
        table_name: &str,
    ) -> Result<Option<Storage<'a, B>>, DatabaseError<B::Error>>
    where
        S: Debug + Serialize + for<'de> Deserialize<'de> + 'static,
    {
        let map_ref = unsafe {
            let temp = self.databases.get(table_name);

            if temp.is_none() {
                return Ok(None);
            }

            temp.unwrap_unchecked()
        };

        map_ref.value().check::<S>()?;

        Ok(Some(Storage::new(map_ref)))
    }

    /// todo
    ///
    /// # Safety
    ///
    /// todo
    pub unsafe fn get_unchecked<'a, S>(&'a self, table_name: &str) -> Storage<'a, B>
    where
        S: Debug + Serialize + for<'de> Deserialize<'de> + 'static,
    {
        let map_ref = self.databases.get(table_name).unwrap_unchecked();

        map_ref.value().check::<S>().unwrap_unchecked();

        Storage::new(map_ref)
    }
}

impl<B: Backend> Drop for Gateway<B> {
    fn drop(&mut self) {
        block_on(unsafe { self.backend.shutdown() });
    }
}
