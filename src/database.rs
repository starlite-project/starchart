#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use crate::backend::Backend;
use std::{any::TypeId, sync::Arc};

/// todo
#[derive(Debug)]
pub struct Database<B: Backend> {
    table_name: String,
    backend: Arc<B>,
    type_id: Option<TypeId>,
}

impl<B: Backend> Database<B> {
    /// todo
    pub(crate) fn new(table_name: String, backend: Arc<B>) -> Self {
        Self {
            table_name,
            backend,
            type_id: None,
        }
    }

    /// todo
    /// 
    /// # Panics
    /// 
    /// todo
    pub fn kind<V>(&self) 
    where V: Serialize + for<'de> Deserialize<'de> + 'static {
        let type_id = TypeId::of::<V>();

        if self.type_id.is_some() && self.type_id != Some(type_id) {
            panic!("expected type_id to match")
        }

        if self.type_id.is_none() {
            Self::set_type_id(&mut self, type_id);
        }
    }

    fn set_type_id(&mut self, type_id: TypeId) {
        self.type_id = Some(type_id);
    }

    /// todo
    /// 
    /// # Panics
    /// 
    /// todo
    /// 
    /// # Errors
    /// 
    /// todo
    pub async fn get<'a, D>(&self, id: &str) -> Result<Option<D>, B::Error>
    where
        D: for<'de> Deserialize<'de> + 'static,
    {
        let type_id = TypeId::of::<D>();

        todo!()
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
