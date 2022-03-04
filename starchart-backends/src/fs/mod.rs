//! The file-system based backends for the starchart crate.

#[cfg(feature = "binary")]
mod binary;
mod error;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "yaml")]
mod yaml;

use std::{
	collections::HashMap,
	fs::File,
	io::{ErrorKind, Read, Write},
	iter::FromIterator,
	path::{Path, PathBuf},
};

use futures_util::future::{err, FutureExt};
use serde::de::IgnoredAny;
use starchart::{
	backend::{
		futures::{
			CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, EnsureTableFuture,
			GetFuture, GetKeysFuture, HasFuture, HasTableFuture, InitFuture, UpdateFuture,
		},
		Backend,
	},
	Entry,
};
// use tokio::fs::OpenOptions;
use tokio::fs::{create_dir_all, read_dir, remove_file, OpenOptions};

pub use self::error::{FsError, FsErrorType};
use self::util::IgnoredData;

/// An fs-based backend for [`starchart`].
#[derive(Debug, Default, Clone)]
#[cfg(feature = "fs")]
pub struct FsBackend<T> {
	transcoder: T,
	base_directory: PathBuf,
}

impl<T: Transcoder> FsBackend<T> {
	/// Creates a new [`FsBackend`].
	///
	/// # Errors
	///
	/// Returns an error if the provided path is not a directory.
	pub fn new<P: AsRef<Path>>(transcoder: T, base_directory: P) -> Result<Self, FsError> {
		let path = base_directory.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError {
				source: None,
				kind: FsErrorType::PathNotDirectory(path),
			})
		} else {
			Ok(Self {
				transcoder,
				base_directory: path,
			})
		}
	}

	/// Base directory for the [`FsBackend`].
	pub fn base_directory(&self) -> &Path {
		&self.base_directory
	}

	/// Extension used for tables.
	pub fn extension(&self) -> &str {
		self.transcoder().extension()
	}

	/// Reference to the [`Transcoder`].
	pub fn transcoder(&self) -> &T {
		&self.transcoder
	}
}

impl<T: Transcoder> Backend for FsBackend<T> {
	type Error = FsError;

	fn init(&self) -> InitFuture<'_, Self::Error> {
		async move {
			let path = self.base_directory();

			let exists = match read_dir(path).await {
				Ok(_) => true,
				Err(e) if e.kind() == ErrorKind::NotFound => false,
				Err(e) => return Err(e.into()),
			};

			if !exists {
				create_dir_all(path).await?;
			}

			Ok(())
		}
		.boxed()
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		async move {
			let path = self
				.base_directory()
				.join(&[table, self.extension()].join("."));

			let res = OpenOptions::new().read(true).open(path).await;

			match res {
				Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
				Err(e) => Err(e.into()),
				Ok(_) => Ok(true),
			}
		}
		.boxed()
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		async move {
			let path = self
				.base_directory()
				.join(&[table, self.extension()].join("."));

			let mut file: File = OpenOptions::new()
				.write(true)
				.create(true)
				.open(path)
				.await?
				.into_std()
				.await;

			let map: HashMap<String, IgnoredData> = HashMap::new();

			let empty_data = self.transcoder().serialize_value(&map)?;

			file.write_all(&empty_data)?;

			Ok(())
		}
		.boxed()
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		let path = self
			.base_directory()
			.join(&[table, self.extension()].join("."));

		remove_file(path)
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
			let path = self
				.base_directory()
				.join(&[table, self.extension()].join("."));
			let file: File = OpenOptions::new()
				.read(true)
				.open(path)
				.await?
				.into_std()
				.await;
			let entries: HashMap<String, IgnoredData> = self.transcoder().deserialize_data(file)?;

			entries.into_keys().map(Ok).collect()
		}
		.boxed()
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		async move {
			let path = self
				.base_directory()
				.join(&[table, self.extension()].join("."));

			let file: File = OpenOptions::new()
				.read(true)
				.open(path)
				.await?
				.into_std()
				.await;
			let mut map: HashMap<String, D> = self.transcoder().deserialize_data(file)?;

			Ok(map.remove(id))
		}
		.boxed()
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		async move {
			let path = self
				.base_directory()
				.join(&[table, self.extension()].join("."));

			let file: File = OpenOptions::new()
				.read(true)
				.open(path)
				.await?
				.into_std()
				.await;

			let map: HashMap<String, IgnoredData> = self.transcoder().deserialize_data(file)?;

			Ok(map.contains_key(id))
		}
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
		async move {
			let path = self
				.base_directory()
				.join(&[table, self.extension()].join("."));

			let mut file: File = OpenOptions::new()
				.write(true)
				.read(true)
				.open(path)
				.await?
				.into_std()
				.await;

			let mut data: HashMap<String, S> = self.transcoder().deserialize_data(&file)?;

			data.insert(id.to_owned(), value.clone());

			let raw = self.transcoder().serialize_value(&data)?;

			file.write_all(&raw)?;

			Ok(())
		}
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
		self.create(table, id, value)
	}

	fn delete<'a, S: Entry>(
		&'a self,
		table: &'a str,
		id: &'a str,
	) -> DeleteFuture<'a, Self::Error> {
		async move {
			let path = self
				.base_directory()
				.join(&[table, self.extension()].join("."));

			let mut file: File = OpenOptions::new()
				.read(true)
				.write(true)
				.open(path)
				.await?
				.into_std()
				.await;

			let mut map: HashMap<String, S> = self.transcoder().deserialize_data(&file)?;
			map.remove(id);

			let data = self.transcoder().serialize_value(&map)?;

			file.write_all(&data)?;

			Ok(())
		}
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

	/// Gives the extension for the current transcoder.
	fn extension(&self) -> &'static str;
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

mod util {
	use serde::{de::IgnoredAny, Deserialize, Serialize};

	/// needed bc [`IgnoredAny`] doesn't implement Serialize, so it can't be used as an Entry.
	#[derive(Debug, Clone, Copy)]
	pub struct IgnoredData;

	impl Serialize for IgnoredData {
		#[allow(clippy::panic_in_result_fn)]
		fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
		{
			unreachable!()
		}
	}

	impl<'de> Deserialize<'de> for IgnoredData {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
		{
			IgnoredAny::deserialize(deserializer)?;
			Ok(Self)
		}
	}
}
