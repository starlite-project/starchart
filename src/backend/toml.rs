use std::{
	fmt::Debug,
	io,
	path::{Path, PathBuf},
};

use super::fs::{FsBackend, FsError};
use crate::Entry;

/// A TOML based backend.
#[derive(Debug, Default, Clone)]
#[cfg(feature = "toml")]
pub struct TomlBackend(PathBuf);

#[cfg(feature = "toml")]
impl TomlBackend {
	/// Create a new [`TomlBackend`].
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

#[cfg(feature = "toml")]
impl FsBackend for TomlBackend {
	const EXTENSION: &'static str = "toml";

	fn from_reader<R, T>(mut rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		let mut output = Vec::new();
		rdr.read_to_end(&mut output).map_err(|_| FsError::Serde)?;
		serde_toml::from_slice(&output).map_err(|_| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		serde_toml::to_vec(value).map_err(|_| FsError::Serde)
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

/// A TOML based backend, that uses pretty printing.
#[derive(Debug, Default, Clone)]
#[cfg(all(feature = "toml", feature = "pretty"))]
pub struct TomlPrettyBackend(PathBuf);

#[cfg(all(feature = "toml", feature = "pretty"))]
impl TomlPrettyBackend {
	/// Create a new [`TomlPrettyBackend`].
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

#[cfg(all(feature = "toml", feature = "pretty"))]
impl FsBackend for TomlPrettyBackend {
	const EXTENSION: &'static str = "toml";

	fn from_reader<R, T>(mut rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		let mut output = String::new();
		rdr.read_to_string(&mut output)?;
		serde_toml::from_str(&output).map_err(|_| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		serde_toml::to_string_pretty(value)
			.map(String::into_bytes)
			.map_err(|_| FsError::Serde)
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

#[cfg(all(test, feature = "toml"))]
mod tests {
	use std::{fmt::Debug, fs, path::PathBuf};

	use serde::{Deserialize, Serialize};
	use static_assertions::assert_impl_all;

	use crate::{
		backend::{Backend, FsError, TomlBackend},
		util::testing::FsCleanup as Cleanup,
		IndexEntry,
	};

	assert_impl_all!(TomlBackend: Backend, Clone, Debug, Default, Send, Sync);

	#[derive(
		Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
	)]
	struct Settings {
		id: u64,
		option: bool,
		value: u8,
	}

	#[cfg(not(tarpaulin_include))]
	impl IndexEntry for Settings {
		type Key = u64;

		fn key(&self) -> Self::Key {
			self.id
		}
	}

	#[test]
	fn new() -> Result<(), FsError> {
		let path = Cleanup::new("new", "toml", true)?;
		let _blank = Cleanup::new("", "toml", true)?;
		let backend = TomlBackend::new(&path)?;

		assert_eq!(backend.0, PathBuf::from(&path));

		let file_path = Cleanup::new("file.txt", "toml", false)?;

		fs::write(&file_path, "Hello, world!")?;

		assert!(TomlBackend::new(&file_path).is_err());

		fs::remove_file(file_path)?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let path = Cleanup::new("init", "toml", false)?;
		let backend = TomlBackend::new(&path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), FsError> {
		let path = Cleanup::new("has_and_create_table", "toml", true)?;
		let backend = TomlBackend::new(&path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), FsError> {
		let path = Cleanup::new("get_keys", "toml", true)?;
		let backend = TomlBackend::new(&path)?;

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
		let path = Cleanup::new("create_and_delete_table", "toml", true)?;
		let backend = TomlBackend::new(&path)?;

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
		let path = Cleanup::new("get_and_create", "toml", true)?;
		let backend = TomlBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend
			.create(
				"table",
				"id",
				&Settings {
					id: 0,
					option: true,
					value: 42,
				},
			)
			.await?; // coverage:ignore-line

		assert_eq!(
			backend.get::<Settings>("table", "id").await?,
			Some(Settings {
				id: 0,
				option: true,
				value: 42
			})
		);

		assert_eq!(backend.get::<u8>("table", "id2").await?, None);

		assert!(backend.create("table", "id", &2_u8).await.is_err());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn update_and_replace() -> Result<(), FsError> {
		let path = Cleanup::new("update_and_replace", "toml", true)?;
		let backend = TomlBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend
			.create(
				"table",
				"id",
				&Settings {
					id: 0,
					option: true,
					value: 42,
				},
			)
			.await?;

		backend
			.update(
				"table",
				"id",
				&Settings {
					id: 0,
					option: false,
					value: 24,
				},
			)
			.await?;

		assert_eq!(
			backend.get::<Settings>("table", "id").await?,
			Some(Settings {
				id: 0,
				option: false,
				value: 24,
			})
		);

		backend
			.replace(
				"table",
				"id",
				&Settings {
					id: 0,
					option: true,
					value: 72,
				},
			)
			.await?;

		assert_eq!(
			backend.get::<Settings>("table", "id").await?,
			Some(Settings {
				id: 0,
				option: true,
				value: 72
			})
		);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn delete() -> Result<(), FsError> {
		let path = Cleanup::new("delete", "toml", true)?;
		let backend = TomlBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		backend.delete("table", "id").await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, None);

		Ok(())
	}
}
