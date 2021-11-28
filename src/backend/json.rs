use std::{
	ffi::OsString,
	fmt::{Debug, Formatter, Result as FmtResult},
	fs::File as StdFile,
	io,
	iter::FromIterator,
	path::{Path, PathBuf},
};

use futures_util::StreamExt;
use thiserror::Error;
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

use super::{
	fs::{FsBackend, FsReadWrite},
	future::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
		HasFuture, HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
	},
	Backend, FsError,
};
use crate::{util::InnerUnwrap, Entry};

#[derive(Debug, Clone, Copy)]
struct JsonRW;

impl FsReadWrite for JsonRW {
	const EXTENSION: &'static str = "json";

	fn from_reader<R, T>(rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		serde_json::from_reader(rdr).map_err(|e| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		serde_json::to_vec(value).map_err(|e| FsError::Serde)
	}
}

/// An error returned from the [`JsonBackend`].
///
/// [`JsonBackend`]: crate::backend::JsonBackend
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub type JsonError = FsError;

/// A JSON based backend, uses [`serde_json`] to read and write files.
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
#[derive(Debug, Default, Clone)]
pub struct JsonBackend {
	inner: FsBackend<JsonRW>,
}

impl JsonBackend {
	/// Creates a new [`JsonBackend`].
	///
	/// # Errors
	///
	/// Returns a [`JsonError::Io`] if the path appears to be a file.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
		Ok(Self {
			inner: FsBackend::new(path, JsonRW)?,
		})
	}
}

impl Backend for JsonBackend {
	type Error = JsonError;

	fn init(&self) -> InitFuture<'_, JsonError> {
		self.inner.init()
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		self.inner.has_table(table)
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		self.inner.create_table(table)
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		self.inner.delete_table(table)
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		self.inner.get(table, id)
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		self.inner.get_keys(table)
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		self.inner.has(table, id)
	}

	fn create<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> CreateFuture<'a, Self::Error>
	where
		S: Entry,
	{
		self.inner.create(table, id, value)
	}

	fn update<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> UpdateFuture<'a, Self::Error>
	where
		S: Entry,
	{
		self.inner.update(table, id, value)
	}

	fn replace<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> ReplaceFuture<'a, Self::Error>
	where
		S: Entry,
	{
		self.inner.replace(table, id, value)
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
		self.inner.delete(table, id)
	}
}

#[cfg(all(test, feature = "json"))]
mod tests {
	use std::{
		ffi::OsStr,
		fmt::Debug,
		fs,
		io::Result as IoResult,
		path::{Path, PathBuf},
	};

	use static_assertions::assert_impl_all;

	use crate::backend::{Backend, JsonBackend, JsonError};

	#[derive(Debug, Clone)]
	pub struct Cleanup(PathBuf);

	impl Cleanup {
		pub fn new(test_name: &str, should_create: bool) -> IoResult<Self> {
			let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.join("target")
				.join("tests")
				.join(test_name);

			if should_create {
				fs::create_dir_all(&path)?;
			}

			Ok(Self(path))
		}
	}

	impl AsRef<Path> for Cleanup {
		fn as_ref(&self) -> &Path {
			self.0.as_ref()
		}
	}

	impl AsRef<OsStr> for Cleanup {
		fn as_ref(&self) -> &OsStr {
			self.0.as_ref()
		}
	}

	impl Drop for Cleanup {
		#[allow(clippy::let_underscore_drop)]
		fn drop(&mut self) {
			let _ = fs::remove_dir_all(&self.0);
		}
	}

	assert_impl_all!(JsonBackend: Backend, Clone, Debug, Default, Send, Sync);

	#[test]
	fn new() -> Result<(), JsonError> {
		let path = Cleanup::new("new", true)?;
		let _blank = Cleanup::new("", true)?;
		let backend = JsonBackend::new(&path)?;

		assert_eq!(backend.inner.base_directory, PathBuf::from(&path));

		let file_path = Cleanup::new("file.txt", false)?;

		fs::write(&file_path, "Hello, world!")?;

		assert!(JsonBackend::new(&file_path).is_err());

		fs::remove_file(file_path)?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), JsonError> {
		let path = Cleanup::new("init", false)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), JsonError> {
		let path = Cleanup::new("has_and_create_table", true)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), JsonError> {
		let path = Cleanup::new("get_keys", true)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1).await?;
		backend.create("table", "id2", &2).await?;

		let mut keys: Vec<String> = backend.get_keys("table").await?;
		let mut expected = vec!["id".to_owned(), "id2".to_owned()];

		keys.sort();
		expected.sort();

		assert_eq!(keys, expected,);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn create_and_delete_table() -> Result<(), JsonError> {
		let path = Cleanup::new("create_and_delete_table", true)?;
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
	async fn get_and_create() -> Result<(), JsonError> {
		let path = Cleanup::new("get_and_create", true)?;
		let backend = JsonBackend::new(&path)?;

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
	async fn update_and_replace() -> Result<(), JsonError> {
		let path = Cleanup::new("update_and_replace", true)?;
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
	async fn delete() -> Result<(), JsonError> {
		let path = Cleanup::new("delete", true)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "id", &1_u8).await?;

		backend.delete("table", "id").await?;

		assert_eq!(backend.get::<u8>("table", "id").await?, None);

		Ok(())
	}
}
