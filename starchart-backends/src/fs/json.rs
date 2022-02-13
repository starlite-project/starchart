use std::io::Read;

use starchart::Entry;

use super::{FsError, Transcoder};

/// A transcoder for the JSON format.
#[derive(Debug, Default, Clone, Copy)]
#[cfg(feature = "json")]
#[must_use = "transcoders do nothing by themselves"]
pub struct JsonTranscoder(bool);

impl JsonTranscoder {
	/// Creates a new [`JsonTranscoder`], optionally using pretty printing.
	pub const fn new(is_pretty: bool) -> Self {
		Self(is_pretty)
	}

	/// Returns whether or not this transcoder uses pretty formatting.
	#[must_use]
	pub const fn is_pretty(self) -> bool {
		self.0
	}
}

impl Transcoder for JsonTranscoder {
	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError> {
		if self.0 {
			Ok(serde_json::to_vec_pretty(value)?)
		} else {
			Ok(serde_json::to_vec(value)?)
		}
	}

	fn deserialize_data<T: Entry, R: Read>(&self, rdr: R) -> Result<T, FsError> {
		Ok(serde_json::from_reader(rdr)?)
	}
}
