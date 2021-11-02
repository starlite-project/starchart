use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// The key trait to be implemented on [`Settings`] to allow an easy way to get keys.
pub trait Key {
    /// The method to transform a [`Key`] into a value.
    fn to_key(&self) -> String;
}

impl<T> Key for T where T: Settings + ToString {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

/// A marker trait for use within the [`Database`].
///
/// This signifies that the type can be stored within a [`Database`].
/// 
/// [`Database`]: crate::Database
pub trait Settings: Serialize + DeserializeOwned + Debug + Send + Sync {}

impl<T> Settings for T where T: Serialize + DeserializeOwned + Debug + Send + Sync {}