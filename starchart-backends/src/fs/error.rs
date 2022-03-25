use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	io::Error as IoError,
	path::PathBuf,
};

/// An error occurred from the [`FsBackend`] or one of it's [`Transcoders`].
///
/// [`FsBackend`]: super::FsBackend
/// [`Transcoders`]: super::Transcoder
#[derive(Debug)]
#[cfg(feature = "fs")]
pub struct FsError {
	pub(super) source: Option<Box<dyn Error + Send + Sync>>,
	pub(super) kind: FsErrorType,
}

impl FsError {
	/// Creates an error from a [`Transcoder`].
	///
	/// [`Transcoder`]: super::Transcoder
	#[must_use]
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

impl From<IoError> for FsError {
	fn from(e: IoError) -> Self {
		Self {
			source: Some(Box::new(e)),
			kind: FsErrorType::Io,
		}
	}
}

impl From<FsError> for starchart::Error {
	fn from(e: FsError) -> Self {
		Self::backend(Box::new(e))
	}
}

#[cfg(feature = "cbor")]
impl From<serde_cbor::Error> for FsError {
	fn from(e: serde_cbor::Error) -> Self {
		Self::serde(Some(Box::new(e)))
	}
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for FsError {
	fn from(e: serde_json::Error) -> Self {
		Self::serde(Some(Box::new(e)))
	}
}

#[cfg(feature = "toml")]
impl From<serde_toml::de::Error> for FsError {
	fn from(e: serde_toml::de::Error) -> Self {
		Self::serde(Some(Box::new(e)))
	}
}

#[cfg(feature = "toml")]
impl From<serde_toml::ser::Error> for FsError {
	fn from(e: serde_toml::ser::Error) -> Self {
		Self::serde(Some(Box::new(e)))
	}
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for FsError {
	fn from(e: serde_yaml::Error) -> Self {
		Self::serde(Some(Box::new(e)))
	}
}

/// The type of [`FsError`] that occurred.
#[derive(Debug)]
#[cfg(feature = "fs")]
#[non_exhaustive]
pub enum FsErrorType {
	/// An IO error occurred.
	Io,
	/// The path provided was not a directory.
	PathNotDirectory(PathBuf),
	/// An error occurred during (de)serialization.
	Serde,
	/// The given file was invalid in some way.
	InvalidFile(PathBuf),
}
