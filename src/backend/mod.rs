//! todo

use async_trait::async_trait;
use serde::Deserialize;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "json")]
pub use self::json::JsonBackend;

#[cfg(feature = "json")]
#[cfg_attr(feature = "json", doc(hidden))]
pub use self::json::JsonError;

/// todo
#[async_trait]
pub trait Backend {
    /// todo
    type Error;

    /// todo
    async fn init(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// todo
    async fn has_table(&self, table: &str) -> Result<bool, Self::Error>;

    /// todo
    async fn create_table(&self, table: &str) -> Result<(), Self::Error>;

    /// todo
    async fn delete_table(&self, table: &str) -> Result<(), Self::Error>;

    /// todo
    async fn get_keys(&self, table: &str) -> Result<Vec<String>, Self::Error>;

    /// todo
    async fn get<D>(&self, table: &str, id: &str) -> Result<D, Self::Error>
    where
        D: for<'de> Deserialize<'de>;

    async fn has(&self, table: &str, id: &str) -> Result<bool, Self::Error>;
}
