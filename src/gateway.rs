//! The base structure to use for starchart.

use crate::{backend::Backend, database::DatabaseError, Database, Settings};
use dashmap::{mapref::one::Ref, DashMap};
use futures::executor::block_on;
use std::{
    any::TypeId,
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    sync::Arc,
};

/// An immutable reference to a [`Database`].
#[must_use]
pub struct DbRef<'a, B>
where
    B: Backend,
{
    inner: Ref<'a, String, Database<B>>,
}

impl<'a, B> DbRef<'a, B>
where
    B: Backend,
{
    fn new(inner: Ref<'a, String, Database<B>>) -> Self {
        Self { inner }
    }

    /// Returns the key of the [`Database`].
    #[must_use]
    pub fn key(&'a self) -> &'a str {
        self.inner.key()
    }

    /// Returns the [`Database`].
    #[must_use]
    pub fn value(&'a self) -> &'a Database<B> {
        self.inner.value()
    }
}

impl<B> Debug for DbRef<'_, B>
where
    B: Backend + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("DbRef")
            .field("inner", self.value())
            .finish()
    }
}

impl<'a, B> Deref for DbRef<'a, B>
where
    B: Backend,
{
    type Target = Database<B>;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

/// The base structure for managing [`Database`]s.
///
/// The inner data is wrapped in an [`Arc`], so cloning
/// is cheap and will allow multiple accesses to the data.
#[derive(Debug)]
pub struct Gateway<B: Backend> {
    backend: Arc<B>,
    databases: Arc<DashMap<String, Database<B>>>,
}

impl<B: Backend> Gateway<B> {
    /// Gives access to the raw [`Backend`] instance.
    ///
    /// # Safety
    ///
    /// Accessing the backend functions directly isn't inheritly unsafe, however
    /// care must be taken to ensure the data isn't modified directly, and
    /// that [`Backend::shutdown`] isn't directly called.
    #[must_use]
    pub unsafe fn backend(&self) -> &B {
        &*self.backend
    }

    /// Creates a new [`Gateway`], and initializes the [`Backend`].
    ///
    /// # Errors
    ///
    /// Any errors that [`Backend::init`] can raise.
    pub async fn new(backend: B) -> Result<Self, B::Error> {
        backend.init().await?;
        Ok(Self {
            backend: Arc::new(backend),
            databases: Arc::default(),
        })
    }

    /// Acquires a [`Database`], uses [`Gateway::get`] first, then [`Gateway::create`]
    /// if a [`Database`] was not found.
    ///
    /// # Generics
    ///
    /// The generic parameter `S` should be whatever type you plan on storing in the [`Database`],
    /// passing an incorrect type will create a runtime error.
    ///
    /// # Errors
    ///
    /// An error will be raised if the type provided is not the same as the type provided
    /// when the database was created.
    pub async fn acquire<S>(
        &self,
        table_name: String,
    ) -> Result<DbRef<'_, B>, DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        let exists = self.get::<S>(&table_name)?;

        if exists.is_some() {
            return Ok(unsafe { exists.unwrap_unchecked() });
        }

        self.create::<S>(table_name).await
    }

    #[allow(clippy::missing_errors_doc)]
    /// Creates a new [`Database`].
    ///
    /// # Generics
    ///
    /// The generic parameter `S` should be whatever type you plan on storing in the [`Database`],
    /// passing an incorrect type will create a runtime error.
    pub async fn create<S>(
        &self,
        table_name: String,
    ) -> Result<DbRef<'_, B>, DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        let type_id = TypeId::of::<S>();

        let database =
            Database::new(table_name.clone(), Arc::clone(&self.backend), type_id).await?;

        self.databases.insert(table_name.clone(), database);

        Ok(unsafe { self.get_unchecked::<S>(&table_name) })
    }

    /// Gets a [`Database`] from the cache.
    /// 
    /// # Generics
    /// 
    /// The generic parameter `S` should be whatever type you plan on storing in the [`Database`],
    /// passing an incorrect type will create a runtime error.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed type does not match the one the database was created with.
    pub fn get<'a, S>(
        &'a self,
        table_name: &str,
    ) -> Result<Option<DbRef<'a, B>>, DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        let map_ref = unsafe {
            let temp = self.databases.get(table_name);

            if temp.is_none() {
                return Ok(None);
            }

            temp.unwrap_unchecked()
        };

        map_ref.value().check::<S>()?;

        Ok(Some(DbRef::new(map_ref)))
    }

    /// Deletes a [`Database`], uses [`Backend::delete_table`] under the hood.
    /// 
    /// # Generics
    /// 
    /// The generic parameter `S` should be whatever type you plan on storing in the [`Database`],
    /// passing an incorrect type will create a runtime error.
    ///
    /// # Errors
    ///
    /// Can return errors from [`Gateway::get`], as well as from [`Backend::delete_table`].
    ///
    /// [`Gateway::get`]: Self::get
    pub async fn delete<S>(&self, table_name: &str) -> Result<(), DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        let table = match self.get::<S>(table_name)? {
            Some(db) => db,
            None => return Ok(()),
        };

        self.backend.delete_table(&table.name).await?;

        self.databases.remove(&table.name);

        Ok(())
    }

    /// Deletes a [`Database`] from the [`Gateway`] without checking if it
    /// exists first.
    /// 
    /// # Generics
    /// 
    /// The generic parameter `S` should be whatever type you plan on storing in the [`Database`],
    /// passing an incorrect type will create a runtime error.
    ///
    /// # Safety
    ///
    /// This uses both [`Result::unwrap_unchecked`] and [`Option::unwrap_unchecked`] under the hood.
    pub async unsafe fn delete_unchecked<S>(&self, table_name: &str)
    where
        S: Settings + 'static,
    {
        let table = self.get_unchecked::<S>(table_name);

        self.backend
            .delete_table(&table.name)
            .await
            .unwrap_unchecked();

        self.databases.remove(&table.name);
    }

    /// Gets a [`Database`] from the cache without verifying that it exists.
    /// 
    /// # Generics
    /// 
    /// The generic parameter `S` should be whatever type you plan on storing in the [`Database`],
    /// passing an incorrect type will create a runtime error.
    ///
    /// # Safety
    ///
    /// This uses both [`Result::unwrap_unchecked`] and [`Option::unwrap_unchecked`] under the hood.
    pub unsafe fn get_unchecked<'a, S>(&'a self, table_name: &str) -> DbRef<'a, B>
    where
        S: Settings + 'static,
    {
        let map_ref = self.databases.get(table_name).unwrap_unchecked();

        map_ref.value().check::<S>().unwrap_unchecked();

        DbRef::new(map_ref)
    }
}

impl<B: Backend> Clone for Gateway<B> {
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
            databases: self.databases.clone(),
        }
    }
}

impl<B: Backend> Drop for Gateway<B> {
    fn drop(&mut self) {
        block_on(unsafe { self.backend.shutdown() });
    }
}
