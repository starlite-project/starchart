use serde_value::{DeserializerError, SerializerError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaError {
	#[error("{0} already exists in the schema")]
	SchemaKindAlreadyExists(String),
	#[error("an unsupported value was found in the schema")]
	UnsupportedValue,
	#[error(transparent)]
	Serialize(#[from] SerializerError),
	#[error(transparent)]
	Deserialize(#[from] DeserializerError),
}
