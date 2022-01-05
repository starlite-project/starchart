use std::{
	fmt::Debug,
	io,
	path::{Path, PathBuf},
};

use super::fs::{FsBackend, FsError};
use crate::Entry;

// A QueryString based backend
#[derive(Debug, Default, Clone)]
#[cfg(feature = "querystring")]
pub struct QueryStringBackend(PathBuf);

#[cfg(feature = "querystring")]
impl QueryStringBackend {
	/// Create a new [`QueryStringBackend`].
	///
	/// # Errors
	///
	/// Returns a [`FsError::PathNotDirectory`] if the given path is not a directory.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FsError> {
		let path = path.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError::PathNotDirectory(path))
		} else {
			Ok(Self(path))
		}
	}
}

#[cfg(feature = "querystring")]
impl FsBackend for QueryStringBackend {
	const EXTENSION: &'static str = "qs";

	fn from_reader<R, T>(mut rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		let mut bytes = Vec::new();
		rdr.read_to_end(&mut bytes).map_err(|_| FsError::Serde)?;
		serde_qs::from_bytes(&bytes).map_err(|_| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		let mut writer = Vec::new();
		serde_qs::to_writer(value, &mut writer).map_err(|_| FsError::Serde)?;
		Ok(writer)
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}
