//! Helpers for creating file-system based backends.

use std::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	io::{self, Read},
	iter::FromIterator,
	path::PathBuf,
};

use tokio::fs;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

use super::{
	futures::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
		HasFuture, HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
	},
	Backend,
};
use crate::{util::InnerUnwrap, Entry};

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
			FsErrorType::FileAlreadyExists { path } => {
				f.write_str("file ")?;
				Display::fmt(&path.display(), f)?;
				f.write_str(" already exists")
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
	/// The file already exists.
	FileAlreadyExists {
		/// The provided path to the file.
		path: PathBuf,
	},
}

macro_rules! handle_io_result {
	($res:expr, $name:ident, $okay:expr) => {
		handle_io_result!($res, $name, $okay, Ok(None))
	};
	($res:expr, $name:ident, $okay:expr, $not_found:expr) => {
		match $res {
			Ok($name) => $okay,
			Err(err) if err.kind() == ::std::io::ErrorKind::NotFound => $not_found,
			Err(e) => Err(<$crate::error::FsError as ::std::convert::From<
				::std::io::Error,
			>>::from(e)),
		}
	};
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
	fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry;

	/// The base directory of the Fs database.
	fn base_directory(&self) -> PathBuf;
}

#[cfg(feature = "fs")]
impl<RW: FsBackend> Backend for RW {
	type Error = FsError;

	fn init(&self) -> InitFuture<'_, FsError> {
		Box::pin(async move {
			if fs::read_dir(&self.base_directory()).await.is_err() {
				fs::create_dir_all(&self.base_directory()).await?;
			}
			Ok(())
		})
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		Box::pin(async move {
			let result = fs::read_dir(util::resolve_path(self.base_directory(), &[table])).await;

			handle_io_result!(result, _val, Ok(true), Ok(false))
		})
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		Box::pin(async move {
			fs::create_dir(util::resolve_path(self.base_directory(), &[table])).await?;

			Ok(())
		})
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		Box::pin(async move {
			if self.has_table(table).await? {
				fs::remove_dir(util::resolve_path(self.base_directory(), &[table])).await?;
			}

			Ok(())
		})
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		Box::pin(async move {
			let mut stream = ReadDirStream::new(
				fs::read_dir(util::resolve_path(self.base_directory(), &[table])).await?,
			);
			let mut output = Vec::new();

			while let Some(raw) = stream.next().await {
				let entry = raw?;

				if entry.file_type().await?.is_dir() {
					continue; // coverage:ignore-line
				}

				// let filename = self.resolve_key(entry.file_name()).ok();
				let filename = util::resolve_key(Self::EXTENSION, &entry.file_name()).ok();

				if filename.is_none() {
					continue; // coverage:ignore-line
				}

				output.push(unsafe { filename.inner_unwrap() });
			}

			Ok(output.into_iter().collect())
		})
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		Box::pin(async move {
			let filename = util::filename(id.to_owned(), Self::EXTENSION);
			let path = util::resolve_path(self.base_directory(), &[table, filename.as_str()]);
			handle_io_result!(fs::File::open(&path).await, file, {
				Ok(Some(self.read_data(file.into_std().await)?))
			})
		})
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		Box::pin(async move {
			let filename = util::filename(id.to_owned(), Self::EXTENSION);
			let path = util::resolve_path(self.base_directory(), &[table, filename.as_str()]);
			handle_io_result!(fs::metadata(&path).await, _val, Ok(true), Ok(false))
		})
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
		Box::pin(async move {
			let filepath = util::filename(id.to_owned(), Self::EXTENSION);

			let path = util::resolve_path(self.base_directory(), &[table, filepath.as_str()]);

			if self.has(table, id).await? {
				return Err(FsError {
					kind: FsErrorType::FileAlreadyExists { path },
					source: None,
				});
			}

			let serialized = self.to_bytes(value)?;

			fs::write(path, serialized).await?;

			Ok(())
		})
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
		Box::pin(async move {
			let serialized = self.to_bytes(value)?;
			let filepath = util::filename(id.to_owned(), Self::EXTENSION);

			let path = util::resolve_path(self.base_directory(), &[table, filepath.as_str()]);

			fs::write(path, serialized).await?;

			Ok(())
		})
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
		Box::pin(async move {
			self.update(table, id, value).await?;

			Ok(())
		})
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
		Box::pin(async move {
			let filename = util::filename(id.to_owned(), Self::EXTENSION);

			fs::remove_file(util::resolve_path(
				self.base_directory(),
				&[table, filename.as_str()],
			))
			.await?;

			Ok(())
		})
	}
}

mod util {
	use std::{
		ffi::OsStr,
		path::{Path, PathBuf},
	};

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

	pub fn resolve_path(mut base: PathBuf, to_join: &[&str]) -> PathBuf {
		for value in to_join {
			base = base.join(value);
		}

		base
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

		fn to_bytes<T>(&self, _: &T) -> Result<Vec<u8>, FsError>
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
