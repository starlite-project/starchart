use std::{
	fmt::Debug,
	io::{self, Cursor},
	path::{Path, PathBuf},
};

use serde_ron::{extensions::Extensions, ser::PrettyConfig};

use super::fs::{FsBackend, FsError};
use crate::Entry;

fn serialize_ron<W: io::Write, V: Entry>(
	writer: &mut W,
	pretty_config: Option<PrettyConfig>,
	value: &V,
) -> Result<(), FsError> {
	let mut s =
		serde_ron::Serializer::new(writer, pretty_config, true).map_err(|_| FsError::Serde)?;
	value.serialize(&mut s).map_err(|_| FsError::Serde)?;
	Ok(())
}

/// A RON based backend.
#[derive(Debug, Default, Clone)]
#[cfg(feature = "ron")]
pub struct RonBackend(PathBuf);

#[cfg(feature = "ron")]
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

#[cfg(feature = "ron")]
impl FsBackend for RonBackend {
	const EXTENSION: &'static str = "ron";

	fn from_reader<R, T>(rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		serde_ron::de::from_reader(rdr).map_err(|_| FsError::Serde)
	}

	#[cfg(not(tarpaulin_include))]
	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		let mut writer = Cursor::new(Vec::new());
		serialize_ron(&mut writer, None, value)?;
		Ok(writer.into_inner())
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

/// A RON based backend, that uses pretty printing.
#[derive(Debug, Default, Clone)]
#[cfg(all(feature = "ron", feature = "pretty"))]
pub struct RonPrettyBackend(PathBuf);

#[cfg(all(feature = "ron", feature = "pretty"))]
impl RonPrettyBackend {
	/// Create a new [`RonPrettyBackend`].
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

#[cfg(all(feature = "ron", feature = "pretty"))]
impl FsBackend for RonPrettyBackend {
	const EXTENSION: &'static str = "ron";

	fn from_reader<R, T>(rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		serde_ron::de::from_reader(rdr).map_err(|_| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		let pretty_config = PrettyConfig::new()
			.indentor("\t".to_owned())
			.extensions(Extensions::all());
		let mut writer = Cursor::new(Vec::new());
		serialize_ron(&mut writer, Some(pretty_config), value)?;
		Ok(writer.into_inner())
	}

	fn base_directory(&self) -> PathBuf {
		self.0.clone()
	}
}

#[cfg(all(test, feature = "ron"))]
mod tests {
	use std::{fmt::Debug, fs, path::PathBuf};

	use static_assertions::assert_impl_all;

	use crate::{
		backend::{Backend, FsError, RonBackend},
		util::testing::FsCleanup as Cleanup,
	};

	assert_impl_all!(RonBackend: Backend, Clone, Debug, Default, Send, Sync);

	#[test]
	fn new() -> Result<(), FsError> {
		let path = Cleanup::new("new", "ron", true)?;
		let _blank = Cleanup::new("", "ron", true)?;
		let backend = RonBackend::new(&path)?;

		assert_eq!(backend.0, PathBuf::from(&path));

		let file_path = Cleanup::new("file.txt", "ron", false)?;

		fs::write(&file_path, "Hello, world!")?;

		assert!(RonBackend::new(&file_path).is_err());

		fs::remove_file(file_path)?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let path = Cleanup::new("init", "ron", false)?;
		let backend = RonBackend::new(&path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), FsError> {
		let path = Cleanup::new("has_and_create_table", "ron", true)?;
		let backend = RonBackend::new(&path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), FsError> {
		let path = Cleanup::new("get_keys", "ron", true)?;
		let backend = RonBackend::new(&path)?;

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
		let path = Cleanup::new("create_and_delete_table", "ron", true)?;
		let backend = RonBackend::new(&path)?;

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
		let path = Cleanup::new("get_and_create", "ron", true)?;
		let backend = RonBackend::new(&path)?;

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
		let path = Cleanup::new("update_and_replace", "ron", true)?;
		let backend = RonBackend::new(&path)?;

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
		let path = Cleanup::new("delete", "ron", true)?;
		let backend = RonBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		backend.delete("table", "id").await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, None);

		Ok(())
	}
}
