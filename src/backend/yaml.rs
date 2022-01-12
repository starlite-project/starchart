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

/// A YAML based backend.
#[derive(Debug, Default, Clone)]
#[cfg(feature = "yaml")]
pub struct YamlBackend(PathBuf);

impl YamlBackend {
	/// Create a new [`YamlBackend`].
	///
	/// # Errors
	///
	/// Returns an [`FsError`] if the given path is not a directory.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FsError> {
		let path = path.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError::path_not_directory(path))
		} else {
			Ok(Self(path))
		}
	}
}

impl FsBackend for YamlBackend {
	const EXTENSION: &'static str = "yaml";

	fn from_reader<R, T>(&self, rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		serde_yaml::from_reader(rdr).map_err(|e| FsError {
			source: Some(Box::new(e)),
			kind: FsErrorType::Deserialization,
		})
	}

	fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		serde_yaml::to_vec(value).map_err(|e| FsError {
			source: Some(Box::new(e)),
			kind: FsErrorType::Serialization,
		})
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

#[cfg(all(test, feature = "yaml"))]
mod tests {
	use std::{fmt::Debug, fs, path::PathBuf};

	use static_assertions::assert_impl_all;

	use crate::{
		backend::{Backend, FsError, YamlBackend},
		util::testing::FsCleanup as Cleanup,
	};

	assert_impl_all!(YamlBackend: Backend, Clone, Debug, Default, Send, Sync);

	#[test]
	fn new() -> Result<(), FsError> {
		let path = Cleanup::new("new", "yaml", true)?;
		let _blank = Cleanup::new("", "yaml", true)?;
		let backend = YamlBackend::new(&path)?;

		assert_eq!(backend.0, PathBuf::from(&path));

		let file_path = Cleanup::new("file.txt", "yaml", false)?;

		fs::write(&file_path, "Hello, world!")?;

		assert!(YamlBackend::new(&file_path).is_err());

		fs::remove_file(file_path)?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let path = Cleanup::new("init", "yaml", false)?;
		let backend = YamlBackend::new(&path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), FsError> {
		let path = Cleanup::new("has_and_create_table", "yaml", true)?;
		let backend = YamlBackend::new(&path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), FsError> {
		let path = Cleanup::new("get_keys", "yaml", true)?;
		let backend = YamlBackend::new(&path)?;

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
		let path = Cleanup::new("create_and_delete_table", "yaml", true)?;
		let backend = YamlBackend::new(&path)?;

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
		let path = Cleanup::new("get_and_create", "yaml", true)?;
		let backend = YamlBackend::new(&path)?;

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
		let path = Cleanup::new("update_and_replace", "yaml", true)?;
		let backend = YamlBackend::new(&path)?;

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
		let path = Cleanup::new("delete", "yaml", true)?;
		let backend = YamlBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		backend.delete("table", "id").await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, None);

		Ok(())
	}
}
