use std::io::Read;

use starchart::Entry;

use super::{FsError, Transcoder};

/// Format types for the [`BinaryTranscoder`].
#[derive(Debug, Clone, Copy)]
#[cfg(feature = "binary")]
#[non_exhaustive]
#[must_use = "binary formats do nothing on their own"]
pub enum BinaryFormat {
	/// The [`Bincode`] format.
	///
	/// [`Bincode`]: serde_bincode
	Bincode,
	/// The [`CBOR`] format.
	///
	/// [`CBOR`]: serde_cbor
	Cbor,
}

/// A transcoder for multiple binary formats.
#[derive(Debug, Clone, Copy)]
#[cfg(feature = "binary")]
#[must_use = "transcoders do nothing by themselves"]
pub struct BinaryTranscoder(BinaryFormat);

impl BinaryTranscoder {
	/// Creates a new [`BinaryTranscoder`].
	pub const fn new(format: BinaryFormat) -> Self {
		Self(format)
	}

	/// Returns the binary format being used by the transcoder.
	pub const fn format(self) -> BinaryFormat {
		self.0
	}

	/// Checks whether the transcoder uses the [`Bincode`] format.
	///
	/// [`Bincode`]: serde_bincode
	#[must_use]
	pub const fn is_bincode(self) -> bool {
		matches!(self.format(), BinaryFormat::Bincode)
	}

	/// Checks whether the transcoder uses the [`CBOR`] format.
	///
	/// [`CBOR`]: serde_cbor
	#[must_use]
	pub const fn is_cbor(self) -> bool {
		matches!(self.format(), BinaryFormat::Cbor)
	}
}

impl Transcoder for BinaryTranscoder {
	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError> {
		match self.format() {
			BinaryFormat::Bincode => Ok(serde_bincode::serialize(value)?),
			BinaryFormat::Cbor => Ok(serde_cbor::to_vec(value)?),
		}
	}

	fn deserialize_data<T: Entry, R: Read>(&self, rdr: R) -> Result<T, FsError> {
		match self.format() {
			BinaryFormat::Bincode => Ok(serde_bincode::deserialize_from(rdr)?),
			BinaryFormat::Cbor => Ok(serde_cbor::from_reader(rdr)?),
		}
	}
}

#[cfg(all(test, feature = "binary"))]
mod tests {
	use std::{fmt::Debug, fs};

	use starchart::backend::Backend;
	use static_assertions::assert_impl_all;

	use crate::fs::{
		transcoders::{BinaryFormat, BinaryTranscoder},
		util::testing::{MockSettings, TestPath, TEST_GUARD},
		FsBackend, FsError,
	};

	assert_impl_all!(BinaryTranscoder: Clone, Copy, Debug, Send, Sync);

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn init() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("new_bin", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Bincode),
			"bin".to_owned(),
			&path,
		)?;

		backend.init().await?;

		assert!(fs::read_dir(&path).is_ok());

		backend.init().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn table_methods() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("table_methods", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Bincode),
			"bin".to_owned(),
			&path,
		)?;

		backend.init().await?;

		assert!(!backend.has_table("table").await?);

		backend.create_table("table").await?;

		assert!(backend.has_table("table").await?);

		backend.delete_table("table").await?;

		assert!(!backend.has_table("table").await?);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_keys_bin() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("get_keys", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Bincode),
			"bin".to_owned(),
			&path,
		)?;

		backend.init().await?;
		backend.create_table("table").await?;

		let mut settings = MockSettings::new();
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
	#[cfg_attr(miri, ignore)]
	async fn get_keys_cbor() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("get_keys", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Cbor),
			"cbor".to_owned(),
			&path,
		)?;

		backend.init().await?;
		backend.create_table("table").await?;

		let mut settings = MockSettings::new();
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
	#[cfg_attr(miri, ignore)]
	async fn get_and_create_bin() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("get_and_create_bin", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Bincode),
			"bin".to_owned(),
			&path,
		)?;

		backend.init().await?;
		backend.create_table("table").await?;

		backend.create("table", "1", &MockSettings::new()).await?;

		assert!(backend.get::<MockSettings>("table", "1").await?.is_some());

		assert!(backend.get::<MockSettings>("table", "2").await?.is_none());

		let settings = MockSettings {
			id: 2,
			..MockSettings::new()
		};

		assert!(backend.create("table", "2", &settings).await.is_ok());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn get_and_create_cbor() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("get_and_create_cbor", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Cbor),
			"cbor".to_owned(),
			&path,
		)?;

		backend.init().await?;
		backend.create_table("table").await?;

		backend.create("table", "1", &MockSettings::new()).await?;

		assert!(backend.get::<MockSettings>("table", "1").await?.is_some());

		assert!(backend.get::<MockSettings>("table", "2").await?.is_none());

		let settings = MockSettings {
			id: 2,
			..MockSettings::new()
		};

		assert!(backend.create("table", "2", &settings).await.is_ok());

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn update_and_delete_bin() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("update_and_delete_bin", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Bincode),
			"bin".to_owned(),
			&path,
		)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = MockSettings::new();

		backend.create("table", "1", &settings).await?;

		settings.opt = None;

		backend.update("table", "1", &settings).await?;

		assert_eq!(backend.get("table", "1").await?, Some(settings));

		backend.delete("table", "1").await?;

		assert_eq!(backend.get::<MockSettings>("table", "1").await?, None);

		Ok(())
	}

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn update_and_delete_cbor() -> Result<(), FsError> {
		let _lock = TEST_GUARD.write().await;
		let path = TestPath::new("update_and_delete_cbor", "binary");
		let backend = FsBackend::new(
			BinaryTranscoder::new(BinaryFormat::Cbor),
			"cbor".to_owned(),
			&path,
		)?;

		backend.init().await?;

		backend.create_table("table").await?;

		let mut settings = MockSettings::new();

		backend.create("table", "1", &settings).await?;

		settings.opt = None;

		backend.update("table", "1", &settings).await?;

		assert_eq!(backend.get("table", "1").await?, Some(settings));

		backend.delete("table", "1").await?;

		assert_eq!(backend.get::<MockSettings>("table", "1").await?, None);

		Ok(())
	}
}
