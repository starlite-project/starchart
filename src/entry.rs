#![allow(clippy::empty_enum)]

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// The key trait to be implemented on [`Entry`] to allow an easy way to get keys.
pub trait Key {
	/// The method to transform a [`Key`] into a value.
	fn to_key(&self) -> String;
}

/// A marker trait for use within the [`Database`].
///
/// This signifies that the type can be stored within a [`Database`].
///
/// [`Database`]: crate::Database
pub trait Entry: Clone + Serialize + DeserializeOwned + Debug + Send + Sync {}

impl<T: Clone + Serialize + DeserializeOwned + Debug + Send + Sync> Entry for T {}

/// An indexable entry, used for any [`Entry`] that can be indexed by a [`Key`] that it owns.
pub trait IndexEntry: Entry {
	/// The [`Key`] type to index by.
	type Key: Key;

	/// Returns the valid key for the database to index from.
	fn key(&self) -> Self::Key;
}
