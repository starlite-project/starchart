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
		util::testing::{FsCleanup, TEST_GUARD},
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
}
