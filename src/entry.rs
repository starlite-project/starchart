use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// The key trait to be implemented on [`Entry`] to allow an easy way to get keys.
pub trait Key {
	/// The method to transform a [`Key`] into a value.
	fn to_key(&self) -> String;
}

impl<T: ToString> Key for T {
	fn to_key(&self) -> String {
		self.to_string()
	}
}

/// A marker trait for use within the [`Gateway`].
///
/// This signifies that the type can be stored within a [`Gateway`].
///
/// [`Gateway`]: crate::Gateway
pub trait Entry: Clone + Serialize + DeserializeOwned + Debug + Default + Send + Sync {}

impl<T: Clone + Serialize + DeserializeOwned + Debug + Default + Send + Sync> Entry for T {}

/// An indexable entry, used for any [`Entry`] that can be indexed by a [`Key`] that it owns.
pub trait IndexEntry: Entry {
	/// The [`Key`] type to index by.
	type Key: Key;

	/// Returns the valid key for the database to index from.
	fn key(&self) -> Self::Key;
}

#[cfg(test)]
mod tests {
	use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

	use serde::{de::DeserializeOwned, Deserialize, Serialize};
	use static_assertions::assert_impl_all;

	use super::{Entry, Key};

	#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
	struct Settings {
		id: u32,
		name: String,
	}

	#[derive(Debug, Clone)]
	struct Keyable {
		inner: String,
	}

	impl Display for Keyable {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			Display::fmt(&self.inner, f)
		}
	}

	assert_impl_all!(
		Settings: Clone,
		Debug,
		Default,
		DeserializeOwned,
		Entry,
		Serialize
	);

	#[test]
	fn to_key() {
		let keyable = Keyable {
			inner: "12345".to_owned(),
		};

		assert_eq!(keyable.to_key(), "12345".to_owned());
	}
}
