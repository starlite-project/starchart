use serde::{Deserialize, Serialize};
use serde_value::{to_value, SerializerError, Value};

use crate::{Entry, IndexEntry};

pub const METADATA_KEY: &str = "__metadata__";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
	table: String,
	value: Value,
}

impl Metadata {
	pub fn new<T: Entry>(table: String) -> Result<Self, SerializerError> {
		Ok(Self {
			table,
			value: to_value(T::default())?,
		})
	}

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
