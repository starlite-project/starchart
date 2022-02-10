use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	path::PathBuf,
};

#[derive(Debug)]
pub struct FsError {
	source: Option<Box<dyn Error + Send + Sync>>,
	kind: FsErrorType,
}

impl FsError {
	pub fn serde(err: Option<Box<dyn Error + Send + Sync>>) -> Self {
		Self {
			source: err,
			kind: FsErrorType::Serde,
		}
	}

	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &FsErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(self) -> (FsErrorType, Option<Box<dyn Error + Send + Sync>>) {
		(self.kind, self.source)
	}
}

impl Display for FsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			FsErrorType::Io => f.write_str("an IO error occurred"),
			FsErrorType::PathNotDirectory(p) => {
				f.write_str("path ")?;
				Display::fmt(&p.display(), f)?;
				f.write_str(" is not a directory")
			}
			FsErrorType::Serde => f.write_str("a (de)serialization error occurred"),
			FsErrorType::InvalidFile(p) => {
				f.write_str("file ")?;
				Display::fmt(&p.display(), f)?;
				f.write_str(" is invalid")
			}
		}
	}
}

impl Error for FsError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source
			.as_ref()
			.map(|err| &**err as &(dyn Error + 'static))
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FsErrorType {
	Io,
	PathNotDirectory(PathBuf),
	Serde,
	InvalidFile(PathBuf),
}
