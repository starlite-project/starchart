use std::io::Read;

use starchart::Entry;

use super::{transcoders::TranscoderFormat, FsError, Transcoder};

/// A transcoder for the JSON format.
#[derive(Debug, Default, Clone, Copy)]
#[cfg(feature = "json")]
#[must_use = "transcoders do nothing by themselves"]
pub struct JsonTranscoder(TranscoderFormat);

impl JsonTranscoder {
	/// Creates a new [`JsonTranscoder`], optionally using pretty printing.
	pub const fn new(format: TranscoderFormat) -> Self {
		Self(format)
	}

	/// Returns whether or not this transcoder uses pretty formatting.
	#[must_use]
	pub const fn is_pretty(self) -> bool {
		matches!(self.0, TranscoderFormat::Pretty)
	}

	/// Returns whether or not this transcoder uses standard formatting.
	#[must_use]
	pub const fn is_standard(self) -> bool {
		!self.is_pretty()
	}

	/// Create a new [`JsonTranscoder`] with prettier file formatting.
	pub const fn pretty() -> Self {
		Self::new(TranscoderFormat::Pretty)
	}

	/// Creates a new [`JsonTranscoder`] with standard file formatting.
	pub const fn standard() -> Self {
		Self::new(TranscoderFormat::Standard)
	}
}

impl Transcoder for JsonTranscoder {
	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError> {
		if self.is_pretty() {
			Ok(serde_json::to_vec_pretty(value)?)
		} else {
			Ok(serde_json::to_vec(value)?)
		}
	}

	fn deserialize_data<T: Entry, R: Read>(&self, rdr: R) -> Result<T, FsError> {
		Ok(serde_json::from_reader(rdr)?)
	}
}

#[cfg(all(test, not(miri)))]
mod tests {
	use std::{fmt::Debug, fs};

	use starchart::backend::Backend;
	use static_assertions::assert_impl_all;

	use crate::{
		fs::{transcoders::JsonTranscoder, FsBackend, FsError},
		testing::{TestPath, TestSettings, TEST_GUARD},
	};

	assert_impl_all!(JsonTranscoder: Clone, Copy, Debug, Send, Sync);

	#[tokio::test]
	async fn init() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("init", "json");
		let backend = FsBackend::new(JsonTranscoder::default(), "json".to_owned(), &path)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	async fn table_methods() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("table_methods", "json");
		let backend = FsBackend::new(JsonTranscoder::default(), "json".to_owned(), &path)?;

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
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("get_keys", "json");
		let backend = FsBackend::new(JsonTranscoder::default(), "json".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = TestSettings::default();
		backend.create("table", "1", &settings).await?;
		settings.id = 2;
		settings.opt = None;
		backend.create("table", "2", &settings).await?;

		let mut keys: Vec<String> = backend.get_keys("table").await?;

		let mut expected = vec!["1".to_owned(), "2".to_owned()];

		keys.sort();
		expected.sort();

		assert_eq!(keys, expected);

		Ok(())
	}

	#[tokio::test]
	async fn get_keys_pretty() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("get_keys_pretty", "json");
		let backend = FsBackend::new(JsonTranscoder::pretty(), "json".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = TestSettings::default();
		backend.create("table", "1", &settings).await?;
		settings.id = 2;
		settings.opt = None;
		backend.create("table", "2", &settings).await?;

		let mut keys: Vec<String> = backend.get_keys("table").await?;

		let mut expected = vec!["1".to_owned(), "2".to_owned()];

		keys.sort();
		expected.sort();

		assert_eq!(keys, expected);

		Ok(())
	}

	#[tokio::test]
	async fn get_and_create() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("get_and_create", "json");
		let backend = FsBackend::new(JsonTranscoder::default(), "json".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;
		backend
			.create("table", "1", &TestSettings::default())
			.await?;

		assert_eq!(
			backend.get::<TestSettings>("table", "1").await?,
			Some(TestSettings::default())
		);

		assert_eq!(backend.get::<TestSettings>("table", "2").await?, None);

		let settings = TestSettings {
			id: 2,
			..TestSettings::default()
		};

		assert!(backend.create("table", "2", &settings).await.is_ok());

		Ok(())
	}

	#[tokio::test]
	async fn get_and_create_pretty() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("get_and_create_pretty", "json");
		let backend = FsBackend::new(JsonTranscoder::pretty(), "json".to_owned(), &path)?;

		backend.init().await?;

		backend.create_table("table").await?;
		backend
			.create("table", "1", &TestSettings::default())
			.await?;

		assert_eq!(
			backend.get::<TestSettings>("table", "1").await?,
			Some(TestSettings::default())
		);

		assert_eq!(backend.get::<TestSettings>("table", "2").await?, None);

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
		let path = TestPath::new("update_and_delete", "json");
		let backend = FsBackend::new(JsonTranscoder::default(), "json".to_owned(), &path)?;

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

	#[tokio::test]
	async fn update_and_delete_pretty() -> Result<(), FsError> {
		let _lock = TEST_GUARD.lock().await;
		let path = TestPath::new("update_and_delete_pretty", "json");
		let backend = FsBackend::new(JsonTranscoder::pretty(), "json".to_owned(), &path)?;

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
