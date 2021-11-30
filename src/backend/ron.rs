use std::{
	fmt::Debug,
	io,
	path::{Path, PathBuf},
};

use super::fs::{FsBackend, FsError};
use crate::Entry;

/// A RON based backend.
#[derive(Debug, Default, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "ron")))]
pub struct RonBackend(PathBuf);

impl RonBackend {
	/// Create a new [`RonBackend`].
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

impl FsBackend for RonBackend {
	const EXTENSION: &'static str = "ron";

	fn from_reader<R, T>(mut rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		let mut output = String::new();
		rdr.read_to_string(&mut output)?;
		ron_sys::from_str(&output).map_err(|_| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		ron_sys::to_string(value)
			.map(String::into_bytes)
			.map_err(|_| FsError::Serde)
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}
