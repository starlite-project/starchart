use std::{
	fmt::Debug,
	io,
	path::{Path, PathBuf},
};

use super::fs::{FsBackend, FsError};
use crate::Entry;

/// A YAML basec backend.
#[derive(Debug, Default, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
pub struct YamlBackend(PathBuf);

impl YamlBackend {
	/// Create a new [`YamlBackend`].
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

impl FsBackend for YamlBackend {
	const EXTENSION: &'static str = "yaml";

	fn from_reader<R, T>(rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		serde_yaml::from_reader(rdr).map_err(|_| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		serde_yaml::to_vec(value).map_err(|_| FsError::Serde)
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}
