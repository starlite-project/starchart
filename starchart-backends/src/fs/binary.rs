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
