use std::io::Read;

use starchart::Entry;

use super::{Transcoder, FsError};

#[derive(Debug, Default, Clone,Copy)]
#[cfg(feature = "toml")]
pub struct TomlTranscoder(bool);

impl TomlTranscoder {
	pub const fn new(is_pretty: bool) -> Self {
		Self(is_pretty)
	}

	pub const fn is_pretty(self) -> bool {
		self.0
	}
}

impl Transcoder for TomlTranscoder {
	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError> {
		if self.0 {
			Ok(serde_toml::to_string_pretty(value).map(String::into_bytes)?)
		} else {
			Ok(serde_toml::to_vec(value)?)
		}
	}

	fn deserialize_data<T: Entry, R: Read>(&self, mut rdr: R) -> Result<T, FsError> {
		let mut output = String::new();
		rdr.read_to_string(&mut output)?;
		Ok(serde_toml::from_str(&output)?)
	}
}
