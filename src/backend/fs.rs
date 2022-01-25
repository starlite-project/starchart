//! Helpers for creating file-system based backends.

use std::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	fs::File as StdFile,
	io::{self, ErrorKind, Read},
	iter::FromIterator,
	path::PathBuf,
};

use futures_util::future::FutureExt;
use tokio::fs;

use super::{
	futures::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
		HasFuture, HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
	},
	Backend,
};
use crate::Entry;

/// An error occurred from an [`FsBackend`].
#[cfg(feature = "fs")]
#[derive(Debug)]
pub struct FsError {
	/// Source error, if there was any.
	pub source: Option<Box<dyn Error + Send + Sync>>,
	/// Type of [`FsError`] that occurred.
	pub kind: FsErrorType,
}

#[cfg(feature = "fs")]
impl FsError {
	/// Immutable reference to the type of error that occurred.
	#[must_use = "retrieving the type has no effect if left unused"]
	pub const fn kind(&self) -> &FsErrorType {
		&self.kind
	}

	/// Consume the error, returning the source error if there is any.
	#[must_use = "consuming the error and retrieving the source has no effect if left unused"]
	pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
		self.source
	}

	/// Consume the error, returning the owned error type and the source error.
	#[must_use = "consuming the error into it's parts has no effect if left unused"]
	pub fn into_parts(self) -> (FsErrorType, Option<Box<dyn Error + Send + Sync>>) {
		(self.kind, self.source)
	}
}

#[cfg(feature = "fs")]
impl Display for FsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match &self.kind {
			FsErrorType::Io => f.write_str("an IO error occurred"),
			FsErrorType::PathNotDirectory { path } => {
				f.write_str("path ")?;
				Display::fmt(&path.display(), f)?;
				f.write_str(" is not a directory")
			}
			FsErrorType::Serde => f.write_str("an error occurred during (de)serialization"),
			FsErrorType::InvalidFile { path } => {
				f.write_str("file ")?;
				Display::fmt(&path.display(), f)?;
				f.write_str(" is invalid")
			}
		}
	}
}

#[cfg(feature = "fs")]
impl Error for FsError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source
			.as_ref()
			.map(|source| &**source as &(dyn Error + 'static))
	}
}

#[cfg(feature = "fs")]
impl From<io::Error> for FsError {
	fn from(err: io::Error) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: FsErrorType::Io,
		}
	}
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for FsError {
	fn from(err: serde_json::Error) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: FsErrorType::Serde,
		}
	}
}

#[cfg(feature = "bincode")]
impl From<serde_bincode::Error> for FsError {
	fn from(err: serde_bincode::Error) -> Self {
		Self {
			source: Some(err),
			kind: FsErrorType::Serde,
		}
	}
}

#[cfg(feature = "bincode")]
impl From<serde_bincode::ErrorKind> for FsError {
	fn from(err: serde_bincode::ErrorKind) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: FsErrorType::Serde,
		}
	}
}

#[cfg(feature = "toml")]
impl From<serde_toml::ser::Error> for FsError {
	fn from(err: serde_toml::ser::Error) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: FsErrorType::Serde,
		}
	}
}

#[cfg(feature = "toml")]
impl From<serde_toml::de::Error> for FsError {
	fn from(err: serde_toml::de::Error) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: FsErrorType::Serde,
		}
	}
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for FsError {
	fn from(err: serde_yaml::Error) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: FsErrorType::Serde,
		}
	}
}

/// The type of [`FsError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
#[cfg(feature = "fs")]
pub enum FsErrorType {
	/// An IO error occurred.
	Io,
	/// The path provided was not a directory.
	PathNotDirectory {
		/// The provided path.
		path: PathBuf,
	},
	/// An error occurred during (de)serialization.
	Serde,
	/// The given file was invalid in some way.
	InvalidFile {
		/// The provided path to the file.
		path: PathBuf,
	},
}

/// The trait for all File System based backends to implement
///
/// This makes it easier to implement different file-system based databases.
#[cfg(feature = "fs")]
pub trait FsBackend: Send + Sync {
	/// The base extension of the files.
	const EXTENSION: &'static str;

	/// Turn a [`Read`]er into an [`Entry`].
	///
	/// # Errors
	///
	/// Implementors should return an error of type [`FsErrorType::Serde`] error upon failure.
	fn read_data<R, T>(&self, rdr: R) -> Result<T, FsError>
	where
		R: Read,
		T: Entry;

	/// Turn a [`Entry`] into a writable [`Vec`].
	///
	/// # Errors
	///
	/// Implementors should return an error of type [`FsErrorType::Serde`] error upon failure.
	fn write_serial<T>(&self, value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry;

	/// The base directory of the Fs database.
	fn base_directory(&self) -> PathBuf;
}

#[cfg(feature = "fs")]
impl<RW: FsBackend> Backend for RW {
	type Error = FsError;

	fn init(&self) -> InitFuture<'_, FsError> {
		async move {
			let path = self.base_directory();
			if fs::read_dir(&path).await.is_err() {
				fs::create_dir_all(&path).await?;
			}

			Ok(())
		}
		.boxed()
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		let mut path = self.base_directory();
		path.push(table);
		fs::read_dir(path)
			.map(|res| match res {
				Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
				Err(e) => Err(e.into()),
				Ok(_) => Ok(true),
			})
			.boxed()
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		let mut path = self.base_directory();
		path.push(table);
		fs::create_dir(path)
			.map(|res| res.map_err(Into::into))
			.boxed()
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		async move {
			let mut path = self.base_directory();
			path.push(table);
			match fs::remove_dir(path).await {
				Err(e) if e.kind() != ErrorKind::NotFound => Err(e.into()),
				_ => Ok(()),
			}
		}
		.boxed()
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		async move {
			let mut path = self.base_directory();
			path.push(table);
			let mut read_dir = fs::read_dir(path).await?;

			let mut output = Vec::new();

			while let Some(entry) = read_dir.next_entry().await? {
				if entry.file_type().await?.is_dir() {
					continue;
				}
				output.push(util::resolve_key(Self::EXTENSION, &entry.file_name()));
			}

			output.into_iter().collect::<Result<I, FsError>>()
		}
		.boxed()
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		async move {
			let filename = util::filename(id.to_owned(), Self::EXTENSION);
			let mut path = self.base_directory();
			path.extend(&[table, filename.as_str()]);
			let file: StdFile = match fs::File::open(&path).await {
				Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
				Err(e) => return Err(e.into()),
				Ok(v) => v.into_std().await,
			};

			Ok(Some(self.read_data(file)?))
		}
		.boxed()
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		async move {
			let filename = util::filename(id.to_owned(), Self::EXTENSION);
			let mut path = self.base_directory();
			path.extend(&[table, filename.as_str()]);
			match fs::metadata(path).await {
				Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
				Err(e) => Err(e.into()),
				Ok(_) => Ok(true),
			}
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
			let filename = util::filename(id.to_owned(), Self::EXTENSION);
			// let path = util::resolve_path(self.base_directory(), &[table, filepath.as_str()]);
			let mut path = self.base_directory();
			path.extend(&[table, filename.as_str()]);

			let serialized = self.write_serial(value)?;

			fs::write(path, serialized).await?;

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
		async move {
			let serialized = self.write_serial(value)?;
			let filepath = util::filename(id.to_owned(), Self::EXTENSION);
			let mut path = self.base_directory();
			path.extend(&[table, filepath.as_str()]);

			fs::write(path, serialized).await?;

			Ok(())
		}
		.boxed()
	}

	fn replace<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> ReplaceFuture<'a, Self::Error>
	where
		S: Entry,
	{
		self.update(table, id, value)
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
		async move {
			let filename = util::filename(id.to_owned(), Self::EXTENSION);

			let mut path = self.base_directory();
			path.extend(&[table, filename.as_str()]);

			match fs::remove_file(path).await {
				Err(e) if e.kind() != ErrorKind::NotFound => Err(e.into()),
				_ => Ok(()),
			}
		}
		.boxed()
	}
}

mod util {
	use std::{ffi::OsStr, path::Path};

	use super::{FsError, FsErrorType};

	pub fn resolve_key(extension: &str, file_name: &OsStr) -> Result<String, FsError> {
		let path_ref: &Path = file_name.as_ref();

		if path_ref.extension().map_or(false, |path| path == extension) {
			path_ref
				.file_stem()
				.ok_or(FsError {
					source: None,
					kind: FsErrorType::InvalidFile {
						path: path_ref.to_path_buf(),
					},
				})
				.map(|raw| raw.to_string_lossy().into_owned())
		} else {
			Err(FsError {
				kind: FsErrorType::InvalidFile {
					path: path_ref.to_path_buf(),
				},
				source: None,
			})
		}
	}

	pub fn filename(file_name: String, extension: &str) -> String {
		file_name + "." + extension
	}
}

#[cfg(all(test, feature = "fs"))]
mod tests {
	use std::{ffi::OsStr, io, path::PathBuf};

	use super::{util, FsBackend, FsError, FsErrorType};
	use crate::Entry;

	#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	struct MockFsBackend;

	#[cfg(not(tarpaulin_include))]
	impl FsBackend for MockFsBackend {
		const EXTENSION: &'static str = "test";

		fn read_data<R, T>(&self, _: R) -> Result<T, FsError>
		where
			R: io::Read,
			T: Entry,
		{
			Err(FsError {
				kind: FsErrorType::Serde,
				source: None,
			})
		}

		fn write_serial<T>(&self, _: &T) -> Result<Vec<u8>, FsError>
		where
			T: Entry,
		{
			Err(FsError {
				kind: FsErrorType::Serde,
				source: None,
			})
		}

		fn base_directory(&self) -> PathBuf {
			PathBuf::new()
				.join(".")
				.join("target")
				.join("tests")
				.join("fs")
		}
	}

	#[test]
	fn resolve_key() -> Result<(), FsError> {
		let key = "foo.test";

		assert_eq!(
			util::resolve_key(MockFsBackend::EXTENSION, OsStr::new(key))?,
			"foo"
		);

		let invalid = "bar.json";

		assert!(util::resolve_key(MockFsBackend::EXTENSION, OsStr::new(invalid)).is_err());

		Ok(())
	}
}
