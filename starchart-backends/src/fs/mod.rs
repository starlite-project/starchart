mod error;

use std::{io::Read, path::{Path, PathBuf}};

use starchart::Entry;

pub use self::error::{FsError, FsErrorType};

#[derive(Debug, Clone)]
pub struct FsBackend<T> {
	transcoder: T,
	extension: String,
	directory: PathBuf,
}

impl<T: Transcoder> FsBackend<T> {
	pub fn new<P: AsRef<Path>>(transcoder: T, extension: String, base_directory: P) -> Result<Self, FsError> {
		todo!()
	}
}

pub trait Transcoder {
	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError>;

	fn deserialize_data<T: Entry, R: Read>(&self, rdr: R) -> Result<T, FsError>;
}
