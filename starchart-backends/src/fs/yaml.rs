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
