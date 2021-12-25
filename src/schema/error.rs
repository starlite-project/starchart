use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum SchemaError {
	#[error("{0} already exists in the schema")]
	SchemaKindAlreadyExists(String),
}
