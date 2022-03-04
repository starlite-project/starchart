use std::io::Read;

use serde::{Deserialize, Serialize};
use starchart::Entry;

use super::{FsError, Transcoder};

/// A transcoder for the [`CBOR`] format.
///
/// [`CBOR`]: serde_cbor
#[derive(Debug, Default, Clone, Copy)]
#[cfg(feature = "cbor")]
#[non_exhaustive]
#[must_use = "transcoders do nothing by themselves"]
pub struct CborTranscoder;

impl CborTranscoder {
	/// Creates a new [`CborTranscoder`].
	pub const fn new() -> Self {
		Self
	}
}

impl Transcoder for CborTranscoder {
	type IgnoredData = CborValue;

	const EXTENSION: &'static str = "cbor";

	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError> {
		Ok(serde_cbor::to_vec(value)?)
	}

	fn deserialize_data<T: Entry, R: Read>(&self, rdr: R) -> Result<T, FsError> {
		Ok(serde_cbor::from_reader(rdr)?)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CborValue(serde_cbor::Value);

impl Default for CborValue {
	fn default() -> Self {
		Self(serde_cbor::Value::Null)
	}
}

#[cfg(all(test, not(miri)))]
mod tests {
	use std::{fmt::Debug, fs};

	use starchart::backend::Backend;
	use static_assertions::assert_impl_all;

	use crate::{
		fs::{transcoders::CborTranscoder, FsBackend, FsError},
		testing::{TestPath, TestSettings, TEST_GUARD},
	};

	assert_impl_all!(CborTranscoder: Clone, Copy, Debug, Send, Sync);

	#[tokio::test]
	async fn init() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("init");
		let backend = FsBackend::new(CborTranscoder::new(), &path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		Ok(())
	}

	#[tokio::test]
	async fn table_methods() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("table_methods");
		let backend = FsBackend::new(CborTranscoder::new(), &path)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		backend.delete_table("table").await?;

		assert!(!backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	async fn get_keys() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock();
		let path = TestPath::new("get_keys");
		let backend = FsBackend::new(CborTranscoder::new(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = TestSettings::default();

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
	async fn get_and_create() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("get_and_create");
		let backend = FsBackend::new(CborTranscoder::new(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;
		backend
			.create("table", "1", &TestSettings::default())
			.await?;

		assert!(backend.get::<TestSettings>("table", "1").await?.is_some());

		assert!(backend.get::<TestSettings>("table", "2").await?.is_none());

		let settings = TestSettings {
			id: 2,
			..TestSettings::default()
		};

		assert!(backend.create("table", "2", &settings).await.is_ok());

		Ok(())
	}

	#[tokio::test]
	async fn update_and_delete() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("update_and_delete");
		let backend = FsBackend::new(CborTranscoder::new(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = TestSettings::default();

		backend.create("table", "1", &settings).await?;

		settings.opt = None;

		backend.update("table", "1", &settings).await?;

		assert_eq!(
			backend.get::<TestSettings>("table", "1").await?,
			Some(settings)
		);

		backend.delete("table", "1").await?;

		assert_eq!(backend.get::<TestSettings>("table", "1").await?, None);

		Ok(())
	}
}
