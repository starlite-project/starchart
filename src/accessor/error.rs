use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub struct AccessorError {
	pub(super) source: Option<Box<dyn Error + Send + Sync>>,
	pub(super) kind: AccessorErrorType,
}

impl Display for AccessorError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			AccessorErrorType::Backend => f.write_str("an error occurred with the backend"),
			#[cfg(feature = "metadata")]
			AccessorErrorType::Metadata { type_and_table } => {
				if let Some((type_name, table_name)) = type_and_table {
					f.write_str("invalid entry was provided, ")?;
					Display::fmt(type_name, f)?;
					f.write_str("does not match the metadata for table ")?;
					Display::fmt(table_name, f)
				} else {
					f.write_str("the `__metadata__` key is restricted")
				}
			}
		}
	}
}

impl Error for AccessorError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn Error + 'static))
	}
}

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum AccessorErrorType {
	Backend,
	#[cfg(feature = "metadata")]
	Metadata {
		type_and_table: Option<(&'static str, String)>,
	},
}
