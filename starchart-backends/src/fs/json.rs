use std::io::Read;

use starchart::Entry;

use super::{FsError, Transcoder, transcoders::TranscoderFormat};

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

	pub const fn is_standard(self) -> bool {
		!self.is_pretty()
	}

	pub const fn pretty() -> Self {
		Self::new(TranscoderFormat::Pretty)
	}

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
