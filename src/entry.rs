use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait Key {}

/// A marker trait for use within the [`Database`].
///
/// This signifies that the type can be stored within a [`Database`].
pub trait Settings: Serialize + DeserializeOwned + Debug + Send + Sync {}

impl<T> Settings for T where T: Serialize + DeserializeOwned + Debug + Send + Sync {}