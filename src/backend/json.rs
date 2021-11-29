use std::{
	fmt::Debug,
	io,
	path::{Path, PathBuf},
};

use super::fs::{FsBackend, FsError};
use crate::Entry;

/// A JSON based backend.
#[derive(Debug, Default, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub struct JsonBackend {
	base_directory: PathBuf,
}

impl JsonBackend {
	/// Create a new [`JsonBackend`].
	/// 
	/// # Errors
	/// 
	/// Returns a [`FsError::PathNotDirectory`] if the given path is not a directory.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FsError> {
		let path = path.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError::PathNotDirectory(path))
		} else {
			Ok(Self {
				base_directory: path,
			})
		}
	}
}

impl FsBackend for JsonBackend {
	const EXTENSION: &'static str = "json";

	fn from_reader<R, T>(rdr: R) -> Result<T, FsError>
	where
		R: io::Read,
		T: Entry,
	{
		serde_json::from_reader(rdr).map_err(|_| FsError::Serde)
	}

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry,
	{
		serde_json::to_vec(value).map_err(|_| FsError::Serde)
	}

	fn base_directory(&self) -> PathBuf {
		self.base_directory.clone()
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

	use crate::backend::{Backend, FsError, JsonBackend};

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
	fn new() -> Result<(), FsError> {
		let path = Cleanup::new("new", true)?;
		let _blank = Cleanup::new("", true)?;
		let backend = JsonBackend::new(&path)?;

		assert_eq!(backend.base_directory, PathBuf::from(&path));

		let file_path = Cleanup::new("file.txt", false)?;

		fs::write(&file_path, "Hello, world!")?;

		assert!(JsonBackend::new(&file_path).is_err());

		fs::remove_file(file_path)?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let path = Cleanup::new("init", false)?;
		let backend = JsonBackend::new(&path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), FsError> {
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
	async fn get_keys() -> Result<(), FsError> {
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

		assert_eq!(keys, expected);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn create_and_delete_table() -> Result<(), FsError> {
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
	async fn get_and_create() -> Result<(), FsError> {
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
	async fn update_and_replace() -> Result<(), FsError> {
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
	async fn delete() -> Result<(), FsError> {
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
