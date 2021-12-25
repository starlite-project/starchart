use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{SchemaError, SchemaValue};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

	pub fn new() -> Self {
		Self {
			inner: HashMap::new(),
		}
	}
}
