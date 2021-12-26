use std::{
	collections::{hash_map::IntoIter, HashMap},
	convert::TryFrom,
	fmt::{Debug, Formatter, Result as FmtResult},
	iter::{Extend, Iterator},
};

use serde::{Deserialize, Serialize};
use serde_value::{to_value, Value};

use super::{SchemaError, SchemaValue};
use crate::Entry;

#[derive(Default, Clone, Serialize, Deserialize)]
#[must_use = "a SchemaMap must be used to make a StarChart"]
pub struct SchemaMap {
	inner: HashMap<String, SchemaValue>,
}

impl SchemaMap {
	pub fn include(mut self, name: String, kind: SchemaValue) -> Result<Self, SchemaError> {
		if let Some((exists, _)) = self.inner.get_key_value(&name) {
			return Err(SchemaError::SchemaKindAlreadyExists(exists.clone()));
		}
		self.inner.insert(name, kind);
		Ok(self)
	}

	pub fn from_entry<T: Entry>() -> Result<Self, SchemaError> {
		let default_entry = T::default();

		let value = to_value(default_entry)?;

		let schema_value = SchemaValue::try_from(value)?;

		if let SchemaValue::Subfolder(map) = schema_value {
			Ok(*map)
		} else {
			Err(SchemaError::UnsupportedValue)
		}
	}

	pub fn new() -> Self {
		Self::with_capacity(0)
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			inner: HashMap::with_capacity(capacity),
		}
	}
}

impl Debug for SchemaMap {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_map().entries(self.inner.iter()).finish()
	}
}

impl Extend<(String, SchemaValue)> for SchemaMap {
	fn extend<T: IntoIterator<Item = (String, SchemaValue)>>(&mut self, iter: T) {
		self.inner.extend(iter);
	}
}

impl IntoIterator for SchemaMap {
	type IntoIter = IntoIter<String, SchemaValue>;
	type Item = (String, SchemaValue);

	fn into_iter(self) -> Self::IntoIter {
		self.inner.into_iter()
	}
}
