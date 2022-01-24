use std::{
	fmt::Debug,
	io,
	path::{Path, PathBuf},
};

use super::{
	fs::{FsBackend, FsError},
	FsErrorType,
};
use crate::Entry;

/// A JSON based backend.
#[derive(Debug, Default, Clone)]
#[cfg(feature = "json")]
pub struct JsonBackend(PathBuf);

impl JsonBackend {
	/// Create a new [`JsonBackend`].
	///
	/// # Errors
	///
	/// Returns an [`FsError`] if the given path is not a directory.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FsError> {
		let path = path.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError {
				source: None,
				kind: FsErrorType::PathNotDirectory { path },
			})
		} else {
			Ok(Self(path))
		}
	}
}

impl FsBackend for JsonBackend {
	const EXTENSION: &'static str = "json";

	fn read_data<R, T>(&self, rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		Ok(serde_json::from_reader(rdr)?)
	}

	fn write_serial<T>(&self, value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		Ok(serde_json::to_vec(value)?)
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

/// A JSON based pretty printing backend.
#[derive(Debug, Default, Clone)]
#[cfg(all(feature = "json", feature = "pretty"))]
pub struct JsonPrettyBackend(PathBuf);

#[cfg(all(feature = "json", feature = "pretty"))]
impl JsonPrettyBackend {
	/// Create a new [`JsonPrettyBackend`].
	///
	/// # Errors
	///
	/// Returns an [`FsError`] if the given path is not a directory.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FsError> {
		let path = path.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError {
				source: None,
				kind: FsErrorType::PathNotDirectory { path },
			})
		} else {
			Ok(Self(path))
		}
	}
}

#[cfg(all(feature = "json", feature = "pretty"))]
impl FsBackend for JsonPrettyBackend {
	const EXTENSION: &'static str = "json";

	fn read_data<R, T>(&self, rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		Ok(serde_json::from_reader(rdr)?)
	}

	fn write_serial<T>(&self, value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		Ok(serde_json::to_vec_pretty(value)?)
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

#[cfg(all(test, feature = "json"))]
mod tests {
	use std::{fmt::Debug, fs, path::PathBuf};

	use static_assertions::assert_impl_all;

	use crate::{
		backend::{Backend, FsError, JsonBackend},
		util::testing::{FsCleanup as Cleanup, TEST_GUARD},
	};

	assert_impl_all!(JsonBackend: Backend, Clone, Debug, Default, Send, Sync);

	#[test]
	#[cfg_attr(miri, ignore)]
	fn new() -> Result<(), FsError> {
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("new", "json", true)?;
		let _blank = Cleanup::new("", "json", true)?;
		let backend = JsonBackend::new(&path)?;

		assert_eq!(backend.0, PathBuf::from(&path));

		let file_path = Cleanup::new("file.txt", "json", false)?;

		fs::write(&file_path, "Hello, world!")?;

		assert!(JsonBackend::new(&file_path).is_err());

		fs::remove_file(file_path)?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("init", "json", false)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), FsError> {
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("has_and_create_table", "json", true)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), FsError> {
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("get_keys", "json", true)?;
		let backend = JsonBackend::new(&path)?;

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
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("create_and_delete_table", "json", true)?;
		let backend = JsonBackend::new(&path)?;

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
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("get_and_create", "json", true)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, Some(1));

		assert_eq!(backend.get::<u8>("table", "id2").await?, None);

		assert!(backend.create("table", "id", &2_u8).await.is_ok());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn update_and_replace() -> Result<(), FsError> {
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("update_and_replace", "json", true)?;
		let backend = JsonBackend::new(&path)?;

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
		let _lock = TEST_GUARD.exclusive();
		let path = Cleanup::new("delete", "json", true)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		backend.delete("table", "id").await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, None);

		Ok(())
	}
}
