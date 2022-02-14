use std::io::Read;

use starchart::Entry;

use super::{FsError, Transcoder};

/// A transcoder for the YAML format.
#[derive(Debug, Default, Clone, Copy)]
#[cfg(feature = "yaml")]
#[non_exhaustive]
#[must_use = "transcoders do nothing by themselves"]
pub struct YamlTranscoder;

impl YamlTranscoder {
	/// Creates a new [`YamlTranscoder`].
	pub const fn new() -> Self {
		Self
	}
}

impl Transcoder for YamlTranscoder {
	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError> {
		Ok(serde_yaml::to_vec(value)?)
	}

	fn deserialize_data<T: Entry, R: Read>(&self, rdr: R) -> Result<T, FsError> {
		Ok(serde_yaml::from_reader(rdr)?)
	}
}

#[cfg(all(test, feature = "yaml"))]
mod tests {
	use std::{fmt::Debug, fs};

	use starchart::backend::Backend;
	use static_assertions::assert_impl_all;

	use crate::fs::{
		transcoders::YamlTranscoder,
		util::testing::{FsCleanup, MockSettings, TEST_GUARD},
		FsBackend, FsError,
	};

	assert_impl_all!(YamlTranscoder: Clone, Copy, Debug, Send, Sync);

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = FsCleanup::new("init", "yaml", false)?;
		let backend = FsBackend::new(YamlTranscoder::new(), "yaml".to_owned(), &path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn has_and_create_table() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = FsCleanup::new("has_and_create_table", "yaml", true)?;
		let backend = FsBackend::new(YamlTranscoder::new(), "yaml".to_owned(), &path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write();
		let path = FsCleanup::new("get_keys", "yaml", true)?;
		let backend = FsBackend::new(YamlTranscoder::new(), "yaml".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = MockSettings::new();

		backend.create("table", "1", &settings).await?;
		settings.id = 2;
		settings.opt = None;
		backend.create("table", "2", &settings).await?;

		let mut keys: Vec<String> = backend.get_keys("table").await?;

		let mut expected = ["1".to_owned(), "2".to_owned()];

		keys.sort();
		expected.sort();

		assert_eq!(keys, expected);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn create_and_delete_table() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = FsCleanup::new("create_and_delete_table", "yaml", true)?;
		let backend = FsBackend::new(YamlTranscoder::new(), "yaml".to_owned(), &path)?;

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
		let _lock = TEST_GUARD.write().await;
		let path = FsCleanup::new("get_and_create", "yaml", true)?;
		let backend = FsBackend::new(YamlTranscoder::new(), "yaml".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;
		backend.create("table", "1", &MockSettings::new()).await?;

		assert!(
			backend.get::<MockSettings>("table", "1").await?.is_some()
		);

		assert!(backend.get::<MockSettings>("table", "2").await?.is_none());

		let settings =MockSettings {
			id: 2,
			..MockSettings::new()
		};

		assert!(backend.create("table", "id", &settings).await.is_ok());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn update() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = FsCleanup::new("update", "yaml", true)?;
		let backend = FsBackend::new(YamlTranscoder::new(), "yaml".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = MockSettings::new();

		backend.create("table", "1", &settings).await?;

		settings.opt = None;

		backend.update("table", "1", &settings).await?;

		assert_eq!(backend.get::<MockSettings>("table", "1").await?, Some(settings));

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn delete() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = FsCleanup::new("delete", "yaml",  true)?;
		let backend = FsBackend::new(YamlTranscoder::new(), "yaml".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		backend.create("table", "1", &MockSettings::new()).await?;

		backend.delete("table", "1").await?;

		assert_eq!(backend.get::<MockSettings>("table", "id").await?, None);

		Ok(())
	}
}
