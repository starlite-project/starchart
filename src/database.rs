#![allow(dead_code)]
#[cfg(doc)]
use crate::Gateway;
use crate::{backend::Backend, Settings};
use std::{any::TypeId, error::Error, fmt::Debug, sync::Arc};
use thiserror::Error;

/// An error that can be returned when setting up a [`Database`].
#[derive(Debug, Error)]
pub enum DatabaseError<E: Error = !> {
    /// An invalid generic type was passed to [`Gateway::get`].
    #[error("an invalid type was passed")]
    InvalidType,
    /// An error occurred from the [`Backend`].
    ///
    /// This will match the error type of the backend.
    #[error(transparent)]
    Backend(#[from] E),
}

/// A database for easily interacting with a [`Backend`].
#[derive(Debug)]
pub struct Database<B: Backend> {
    table_name: String,
    backend: Arc<B>,
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

    pub async fn delete(&self) -> Result<(), DatabaseError<B::Error>> {
        self.backend.delete_table(&self.table_name).await?;

        Ok(())
    }

    pub(crate) async fn new(
        table_name: String,
        backend: Arc<B>,
        type_id: TypeId,
    ) -> Result<Self, B::Error> {
        backend.ensure_table(&table_name).await?;

        Ok(Self {
            table_name,
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
            table_name: self.table_name.clone(),
            type_id: self.type_id,
        }
    }
}
