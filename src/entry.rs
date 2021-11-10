use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

// /// The key trait to be implemented on [`Entity`] to allow an easy way to get keys.
// pub trait Key {
//     /// The method to transform a [`Key`] into a value.
//     fn to_key(&self) -> String;
// }

/// A marker trait for use within the [`Database`].
///
/// This signifies that the type can be stored within a [`Database`].
///
/// [`Database`]: crate::Database
pub trait Entity: Serialize + DeserializeOwned + Debug + Send + Sync {
    /// Returns a valid key for the database to index from.
    fn to_key(&self) -> String;
}
