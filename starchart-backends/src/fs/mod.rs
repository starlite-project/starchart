//! The file-system based backends for the starchart crate.

#[cfg(feature = "binary")]
mod binary;
mod error;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "toml")]
mod toml;
mod util;
#[cfg(feature = "yaml")]
mod yaml;

use std::{
	io::{ErrorKind, Read},
	iter::FromIterator,
	path::{Path, PathBuf},
};

use futures_util::future::{err, FutureExt};
use starchart::{
	backend::{
		futures::{
			CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture,
			GetKeysFuture, HasFuture, HasTableFuture, InitFuture, UpdateFuture,
		},
		Backend,
	},
	Entry,
};
use tokio::fs;

pub use self::error::{FsError, FsErrorType};

/// An fs-based backend for the starchart crate.
#[derive(Debug, Clone)]
#[cfg(feature = "fs")]
pub struct FsBackend<T> {
	transcoder: T,
	extension: String,
	base_directory: PathBuf,
}

impl<T: Transcoder> FsBackend<T> {
	/// Creates a new [`FsBackend`].
	///
	/// # Errors
	///
	/// Returns an error if the provided path is not a directory.
	pub fn new<P: AsRef<Path>>(
		transcoder: T,
		extension: String,
		base_directory: P,
	) -> Result<Self, FsError> {
		let path = base_directory.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError {
				source: None,
				kind: FsErrorType::PathNotDirectory(path),
			})
		} else {
			Ok(Self {
				transcoder,
				extension,
				base_directory: path,
			})
		}
	}

	/// Returns the base directory for the [`FsBackend`].
	pub fn base_directory(&self) -> &Path {
		&self.base_directory
	}

	/// Returns the currently used extension for the [`FsBackend`].
	pub fn extension(&self) -> &str {
		&self.extension
	}

	/// Returns a reference to the current [`Transcoder`].
	pub fn transcoder(&self) -> &T {
		&self.transcoder
	}
}

impl<T: Transcoder> Backend for FsBackend<T> {
	type Error = FsError;

	fn init(&self) -> InitFuture<'_, Self::Error> {
		async move {
			let path = self.base_directory();
			let exists = match fs::read_dir(path).await {
				Ok(_) => true,
				Err(e) if e.kind() == ErrorKind::NotFound => false,
				Err(e) => return Err(e.into()),
			};

			if !exists {
				fs::create_dir_all(path).await?;
			}

			Ok(())
		}
		.boxed()
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		let path = self.base_directory().join(table);
		fs::read_dir(path)
			.map(|res| match res {
				Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
				Err(e) => Err(e.into()),
				Ok(_) => Ok(true),
			})
			.boxed()
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		let path = self.base_directory().join(table);
		fs::create_dir(path)
			.map(|res| res.map_err(Into::into))
			.boxed()
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		let path = self.base_directory().join(table);
		fs::remove_dir(path)
			.map(|res| match res {
				Err(e) if e.kind() != ErrorKind::NotFound => Err(e.into()),
				_ => Ok(()),
			})
			.boxed()
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		async move {
			let path = self.base_directory().join(table);
			let mut read_dir = fs::read_dir(&path).await?;

			let mut output = Vec::new();
			while let Some(entry) = read_dir.next_entry().await? {
				if entry.file_type().await?.is_dir() {
					continue;
				}

				output.push(util::resolve_key(self.extension(), &entry.file_name()));
			}

			output.into_iter().collect::<Result<I, Self::Error>>()
		}
		.boxed()
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		async move {
			let filename = [id, self.extension()].join(".");
			let mut path = self.base_directory().to_path_buf();
			path.extend(&[table, filename.as_str()]);
			let file: std::fs::File = match fs::File::open(&path).await {
				Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
				Err(e) => return Err(e.into()),
				Ok(v) => v.into_std().await,
			};

			Ok(Some(self.transcoder().deserialize_data(file)?))
		}
		.boxed()
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		let filename = [id, self.extension()].join(".");
		let mut path = self.base_directory().to_path_buf();
		path.extend(&[table, filename.as_str()]);
		fs::metadata(path)
			.map(|res| match res {
				Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
				Err(e) => Err(e.into()),
				Ok(_) => Ok(true),
			})
			.boxed()
	}

	fn create<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> CreateFuture<'a, Self::Error>
	where
		S: Entry,
	{
		let filename = [id, self.extension()].join(".");
		let mut path = self.base_directory().to_path_buf();
		path.extend(&[table, filename.as_str()]);

		let serialized = match self.transcoder().serialize_value(value) {
			Ok(v) => v,
			Err(e) => return err(e).boxed(),
		};

		fs::write(path, serialized)
			.map(|res| res.map_err(Into::into))
			.boxed()
	}

	fn update<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> UpdateFuture<'a, Self::Error>
	where
		S: Entry,
	{
		let serialized = match self.transcoder().serialize_value(value) {
			Ok(v) => v,
			Err(e) => return err(e).boxed(),
		};

		let filepath = [id, self.extension()].join(".");
		let mut path = self.base_directory().to_path_buf();
		path.extend(&[table, filepath.as_str()]);

		fs::write(path, serialized)
			.map(|res| res.map_err(Into::into))
			.boxed()
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
		let filename = [id, self.extension()].join(".");
		let mut path = self.base_directory().to_path_buf();
		path.extend(&[table, filename.as_str()]);
		fs::remove_file(path)
			.map(|res| match res {
				Err(e) if e.kind() != ErrorKind::NotFound => Err(e.into()),
				_ => Ok(()),
			})
			.boxed()
	}
}

/// The transcoder trait for transforming data for the [`FsBackend`].
#[cfg(feature = "fs")]
pub trait Transcoder: Send + Sync {
	/// Serializes a value into a [`Vec<u8>`] for writing to a file.
	///
	/// # Errors
	///
	/// Any errors from the transcoder should use [`FsError::serde`] to return properly.
	fn serialize_value<T: Entry>(&self, value: &T) -> Result<Vec<u8>, FsError>;

	/// Deserializes data into the provided type.
	///
	/// # Errors
	///
	/// Any errors from the transcoder should use [`FsError::serde`] to return properly.
	fn deserialize_data<T: Entry, R: Read>(&self, rdr: R) -> Result<T, FsError>;
}

/// The transcoders for the [`FsBackend`].
pub mod transcoders {
	#[cfg(feature = "binary")]
	pub use super::binary::{BinaryFormat, BinaryTranscoder};
	#[cfg(feature = "json")]
	pub use super::json::JsonTranscoder;
	#[cfg(feature = "toml")]
	pub use super::toml::TomlTranscoder;
	#[cfg(feature = "yaml")]
	pub use super::yaml::YamlTranscoder;

	/// Transcoder formats for supported transcoders to use.
	#[cfg(any(feature = "toml", feature = "json"))]
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum TranscoderFormat {
		/// Standard formatting, this is the default.
		Standard,
		/// Prettier formatting, this uses more file space but is also more human-readable.
		Pretty,
	}

	#[cfg(any(feature = "toml", feature = "json"))]
	impl Default for TranscoderFormat {
		fn default() -> Self {
			Self::Standard
		}
	}
}
