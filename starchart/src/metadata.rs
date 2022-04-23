//! Items for assisting in metadata.

use serde::{Deserialize, Serialize};
use serde_value::{to_value, SerializerError, Value};

use crate::{Entry, IndexEntry};

/// The key for the metadata tables.
pub const METADATA_KEY: &str = "__metadata__";

/// The metadata struct, uses a [`Value`] to ensure that the tables don't need matching data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
	/// The table the metadata exists for.
	table: String,
	/// The raw metadata.
	value: Value,
}

impl Metadata {
	/// Creates a new metadata from `T`.
	pub fn new<T: Entry>(table: String) -> Result<Self, SerializerError> {
		Ok(Self {
			table,
			value: to_value(T::default())?,
		})
	}

	/// Checks to see if the raw metadata matches the provided entry.
	pub fn is<T: Entry>(&self) -> bool {
		// FIXME: remove this clone somehow
		self.value.clone().deserialize_into::<T>().is_ok()
	}
}

impl IndexEntry for Metadata {
	type Key = String;

	fn key(&self) -> &Self::Key {
		&self.table
	}
}

impl Default for Metadata {
	fn default() -> Self {
		Self {
			table: self::METADATA_KEY.to_owned(),
			value: Value::Unit,
		}
	}
}
