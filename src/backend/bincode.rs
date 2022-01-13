use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	io,
	path::{Path, PathBuf},
};

use serde_bincode::{DefaultOptions, Options};

use super::{
	fs::{FsBackend, FsError},
	FsErrorType,
};
use crate::Entry;

/// A Binary format based backend.
#[cfg(feature = "bincode")]
pub struct BincodeBackend<O>(PathBuf, O);

impl BincodeBackend<DefaultOptions> {
	/// Create a new [`BincodeBackend`] with the [`DefaultOptions`].
	///
	/// # Errors
	///
	/// Returns an [`FsError`] if the given path is not a directory.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FsError> {
		Self::with_options(path, DefaultOptions::new())
	}
}

impl<O: Options> BincodeBackend<O> {
	/// Creates a new [`BincodeBackend`] with the specified [`Options`].
	///
	/// # Errors
	///
	/// Returns an [`FsError`] if the given path is not a directory.
	pub fn with_options<P: AsRef<Path>>(path: P, options: O) -> Result<Self, FsError> {
		let path = path.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError::path_not_directory(path))
		} else {
			Ok(Self(path, options))
		}
	}
}

impl<O> Debug for BincodeBackend<O> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("BincodeBackend")
			.field("path", &self.0)
			.finish_non_exhaustive()
	}
}

impl Default for BincodeBackend<DefaultOptions> {
	fn default() -> Self {
		Self(PathBuf::default(), DefaultOptions::default())
	}
}

impl<O: Options + Copy> Clone for BincodeBackend<O> {
	fn clone(&self) -> Self {
		Self(self.0.clone(), self.1)
	}
}

unsafe impl<O: Options + Copy> Send for BincodeBackend<O> {}
unsafe impl<O: Options + Copy> Sync for BincodeBackend<O> {}

// We use copy because Options is sealed and all the implementors implement copy
impl<O: Options + Send + Sync + Copy> FsBackend for BincodeBackend<O> {
	const EXTENSION: &'static str = "bin";

	fn from_reader<R, T>(&self, rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		self.1.deserialize_from(rdr).map_err(|e| FsError {
			kind: FsErrorType::Deserialization,
			source: Some(e),
		})
	}

	fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		self.1.serialize(value).map_err(|e| FsError {
			kind: FsErrorType::Serialization,
			source: Some(e),
		})
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

#[cfg(all(test, feature = "bincode"))]
mod tests {
	use std::{fmt::Debug, fs, path::PathBuf};

	use serde_bincode::DefaultOptions;
	use static_assertions::assert_impl_all;

	use crate::{
		backend::{Backend, BincodeBackend, FsError},
		util::testing::FsCleanup as Cleanup,
	};

	assert_impl_all!(BincodeBackend<DefaultOptions>: Backend, Clone, Debug, Default, Send, Sync);

	#[test]
	fn new() -> Result<(), FsError> {
		let path = Cleanup::new("new", "bincode", true)?;
		let _blank = Cleanup::new("", "bincode", true)?;
		let backend = BincodeBackend::new(&path)?;

		assert_eq!(backend.0, PathBuf::from(&path));

		let file_path = Cleanup::new("file.txt", "bincode", false)?;

		fs::write(&file_path, "Hello, world!")?;

		assert!(BincodeBackend::new(&file_path).is_err());

		fs::remove_file(file_path)?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let path = Cleanup::new("init", "bincode", false)?;
		let backend = BincodeBackend::new(&path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), FsError> {
		let path = Cleanup::new("has_and_create_table", "bincode", true)?;
		let backend = BincodeBackend::new(&path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), FsError> {
		let path = Cleanup::new("get_keys", "bincode", true)?;
		let backend = BincodeBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1).await?;
		backend.create("table", "id2", &2).await?;

		let mut keys: Vec<String> = backend.get_keys("table").await?;
		let mut expected = vec!["id".to_owned(), "id2".to_owned()];

		keys.sort();
		expected.sort();

		assert_eq!(keys, expected);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn create_and_delete_table() -> Result<(), FsError> {
		let path = Cleanup::new("create_and_delete_table", "bincode", true)?;
		let backend = BincodeBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		backend.delete_table("table").await?;

		assert!(!backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_and_create() -> Result<(), FsError> {
		let path = Cleanup::new("get_and_create", "bincode", true)?;
		let backend = BincodeBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, Some(1));

		assert_eq!(backend.get::<u8>("table", "id2").await?, None);

		assert!(backend.create("table", "id", &2_u8).await.is_err());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn update_and_replace() -> Result<(), FsError> {
		let path = Cleanup::new("update_and_replace", "bincode", true)?;
		let backend = BincodeBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		backend.update("table", "id", &2_u8).await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, Some(2));

		backend.replace("table", "id", &3_u8).await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, Some(3));

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn delete() -> Result<(), FsError> {
		let path = Cleanup::new("delete", "bincode", true)?;
		let backend = BincodeBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		backend.delete("table", "id").await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, None);

		Ok(())
	}
}
