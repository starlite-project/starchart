#![allow(dead_code)]
use crate::backend::Backend;
use serde::{Deserialize, Serialize};
use std::{any::TypeId, error::Error, fmt::Debug, sync::Arc};
use thiserror::Error;

/// todo
#[derive(Debug, Error)]
pub enum DatabaseError<E: Error = crate::error::UnknownError> {
    /// todo
    #[error("an invalid type was passed")]
    InvalidType,
    /// todo
    #[error(transparent)]
    Backend(#[from] E),
    /// todo
    #[error("the value this database holds has not been set")]
    ValueNotSet,
    /// todo
    #[error("this database has already been setup")]
    DatabaseSetup,
}

/// todo
#[derive(Debug)]
pub struct Database<B: Backend> {
    table_name: String,
    backend: Arc<B>,
    type_id: Option<TypeId>,
    setup: bool,
}

impl<B: Backend> Database<B> {
    /// todo
    pub(crate) fn new(table_name: String, backend: Arc<B>) -> Self {
        Self {
            table_name,
            backend,
            type_id: None,
            setup: false,
        }
    }

    pub(crate) async fn setup<S>(&mut self) -> Result<(), DatabaseError<B::Error>>
    where
        S: Serialize + for<'de> Deserialize<'de> + 'static,
    {
        if self.setup {
            return Err(DatabaseError::DatabaseSetup);
        }

        let type_id = TypeId::of::<S>();

        self.type_id = Some(type_id);

        self.backend.ensure_table(&self.table_name).await?;

        Ok(())
    }

    pub(crate) fn check<S>(&self) -> Result<(), DatabaseError<B::Error>>
    where
        S: Debug + Serialize + for<'de> Deserialize<'de> + 'static,
    {
        if self.type_id.is_none() {
            return Err(DatabaseError::ValueNotSet);
        }

        let held_type = unsafe { self.type_id.unwrap_unchecked() };

        let type_of_val = TypeId::of::<S>();

        if type_of_val != held_type {
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
            setup: self.setup,
        }
    }
}
