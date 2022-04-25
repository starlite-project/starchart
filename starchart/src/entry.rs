//! Marker traits that the crate uses.
//!
//! All traits are either optional, or have blanket implementations.

use std::{borrow::Cow, fmt::Debug, ops::Deref};

use serde::{de::DeserializeOwned, Serialize};

/// A dynamic/static key, allowing for keys that don't need to be allocated
/// to be used along side fully allocated [`String`]s.
#[derive(Debug, Clone)]
pub struct Key(Cow<'static, str>);

impl Key {
	/// Create a new key from anything that implements [`ToString`].
	pub fn new<T: ToString>(x: &T) -> Self {
		Self::from(x.to_string())
	}
}

impl Deref for Key {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&*self.0
	}
}

impl From<&'static str> for Key {
	fn from(x: &'static str) -> Self {
		Self(x.into())
	}
}

impl From<String> for Key {
	fn from(x: String) -> Self {
		Self(x.into())
	}
}

impl From<Key> for String {
	fn from(x: Key) -> Self {
		x.0.into_owned()
	}
}

impl PartialEq for Key {
	fn eq(&self, other: &Self) -> bool {
		*self.0 == *other.0
	}
}

impl PartialEq<str> for Key {
	fn eq(&self, other: &str) -> bool {
		&*self.0 == other
	}
}

impl Eq for Key {}

/// A marker trait for use within the [`Starchart`].
///
/// This signifies that the type can be stored within a [`Starchart`].
///
/// [`Starchart`]: crate::Starchart
pub trait Entry: Clone + Serialize + DeserializeOwned + Debug + Default + Send + Sync {}

impl<T: Clone + Serialize + DeserializeOwned + Debug + Default + Send + Sync> Entry for T {}

/// An indexable entry, used for any [`Entry`] that can be indexed by a [`Key`] that it owns.
pub trait IndexEntry: Entry {
	/// Returns the valid key for the database to index from.
	fn key(&self) -> Key;
}
