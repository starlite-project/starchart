use std::collections::HashMap;

use super::{SchemaError, SchemaValue};

#[derive(Debug, Clone)]
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
}
