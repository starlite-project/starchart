use crate::{backend::Backend, Settings};
use std::{any::TypeId, error::Error, fmt::Debug, sync::Arc};
use thiserror::Error;

/// An error that can be returned when setting up a [`Database`].
#[derive(Debug, Error)]
pub enum DatabaseError<E: Error = !> {
    /// An invalid generic type was passed to [`Gateway::get`].
    ///
    /// [`Gateway::get`]: crate::gateway::Gateway::get
    #[error("an invalid type was passed")]
    InvalidType,
    /// An error occurred from the [`Backend`].
    ///
    /// This will match the error type of the backend.
    #[error(transparent)]
    Backend(#[from] E),
    /// An expected value wasn't found in the database.
    #[error("the expected value doesn't exist in the database")]
    ValueDoesntExist,
}

/// A database for easily interacting with a [`Backend`].
#[derive(Debug)]
pub struct Database<B: Backend> {
    pub(crate) name: String,
    pub(crate) backend: Arc<B>,
    type_id: TypeId,
}

impl<B: Backend> Database<B> {
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

    /// Gets a value from the [`Database`].
    ///
    /// # Errors
    ///
    /// Returns an error if the passed type does not match the type this [`Database`] was created with,
    /// or if [`Backend::get`] returned an error.
    pub async fn get<S>(&self, key: &str) -> Result<Option<S>, DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        self.check::<S>()?;

        let value = self.backend.get(&self.name, key).await?;

        Ok(value)
    }

    /// Sets a value in the [`Database`], overwriting whatever was there before.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed type does not match the type this [`Database`] was created with,
    /// or if [`Backend::replace`] or [`Backend::create`] returned an error.
    pub async fn set<S>(&self, key: &str, value: &S) -> Result<(), DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        self.check::<S>()?;

        if self.backend.has(&self.name, key).await? {
            self.backend.replace(&self.name, key, value).await?;
        } else {
            self.backend.create(&self.name, key, value).await?;
        }

        Ok(())
    }

    /// Updates a value in place.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed type does not match the type this [`Database`] was created with,
    /// if the value doesn't exist in the [`Database`], or if [`Backend::update`] returned an error.
    pub async fn update<S>(&self, key: &str, value: &S) -> Result<(), DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        self.check::<S>()?;

        if self.backend.has(&self.name, key).await? {
            self.backend.update(&self.name, key, value).await?;
        } else {
            return Err(DatabaseError::ValueDoesntExist);
        }

        Ok(())
    }

    /// Replaces a value in place.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed type does not match the type this [`Database`] was created with,
    /// or if [`Backend::replace`] returned an error.
    pub async fn replace<S>(&self, key: &str, value: &S) -> Result<(), DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        self.check::<S>()?;

        if self.backend.has(&self.name, key).await? {
            self.backend.replace(&self.name, key, value).await?;
        } else {
            return Err(DatabaseError::ValueDoesntExist);
        }

        Ok(())
    }

    /// Deletes a value from the [`Database`].
    ///
    /// # Errors
    ///
    /// Returns an error if the passed type does not match the type this [`Database`] was created with,
    /// or if [`Backend::delete`] returned an error.
    pub async fn delete<S>(&self, key: &str) -> Result<(), DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        self.check::<S>()?;

        self.backend.delete(&self.name, key).await?;

        Ok(())
    }

    pub(crate) async fn new(
        table_name: String,
        backend: Arc<B>,
        type_id: TypeId,
    ) -> Result<Self, B::Error> {
        backend.ensure_table(&table_name).await?;

        Ok(Self {
            name: table_name,
            backend,
            type_id,
        })
    }

    pub(crate) fn check<S>(&self) -> Result<(), DatabaseError<B::Error>>
    where
        S: Settings + 'static,
    {
        let type_of_val = TypeId::of::<S>();

        if type_of_val != self.type_id {
            return Err(DatabaseError::InvalidType);
        }

        Ok(())
    }
}

impl<B: Backend> Clone for Database<B> {
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
            name: self.name.clone(),
            type_id: self.type_id,
        }
    }
}
