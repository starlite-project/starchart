//! Helpers for creating file-system based backends.

use std::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	fs::File as StdFile,
	io::{self, Read},
	iter::FromIterator,
	path::{Path, PathBuf},
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
	pub(super) source: Option<Box<dyn Error + Send + Sync>>,
	pub(super) kind: FsErrorType,
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

	#[inline]
	pub(super) fn io(err: io::Error) -> Self {
		Self {
			source: Some(Box::new(err)),
			kind: FsErrorType::Io,
		}
	}

	#[inline]
	pub(super) fn path_not_directory(path: PathBuf) -> Self {
		Self {
			source: None,
			kind: FsErrorType::PathNotDirectory { path },
		}
	}

	/// A shortcut for easily creating a serialization error.
	#[must_use]
	#[inline]
	pub fn serialization(err: Option<Box<dyn Error + Send + Sync>>) -> Self {
		Self {
			source: err,
			kind: FsErrorType::Serialization,
		}
	}

	/// A shortcut for easily creating a deserialization error.
	#[must_use]
	#[inline]
	pub fn deserialization(err: Option<Box<dyn Error + Send + Sync>>) -> Self {
		Self {
			source: err,
			kind: FsErrorType::Deserialization,
		}
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
			FsErrorType::Serialization => f.write_str("a serialization error occurred"),
			FsErrorType::Deserialization => f.write_str("a deserialization error occurred"),
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

#[cfg(test)]
impl From<io::Error> for FsError {
	fn from(e: io::Error) -> Self {
		Self::io(e)
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
	/// An error occurred during serialization.
	Serialization,
	/// An error occurred during deserialization.
	Deserialization,
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
			Err(e) => Err($crate::error::FsError::io(e)),
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
	/// Implementors should return an error with [`FsError::serialization`] error upon failure.
	fn from_reader<R, T>(&self, rdr: R) -> Result<T, FsError>
	where
		R: Read,
		T: Entry;

	/// Turn a [`Entry`] into a writable [`Vec`].
	///
	/// # Errors
	///
	/// Implementors should return an error with [`FsError::deserialization`] error upon failure.
	fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry;

	/// Turns a key into a valid filename, with the extension.
	#[must_use]
	fn filename(id: &str) -> String {
		id.to_owned() + "." + Self::EXTENSION
	}

	/// Resolves the path of a file.
	fn resolve_path<P: AsRef<Path>>(&self, path: &[P]) -> PathBuf {
		let mut base = self.base_directory();

		for value in path {
			base = base.join(value);
		}

		base // coverage:ignore-line
	}

	/// Turns a filename into a valid key.
	///
	/// # Errors
	///
	/// Returns an [`FsError`] when the file path does not have the correct extension.
	fn resolve_key<P: AsRef<Path>>(&self, p: P) -> Result<String, FsError> {
		let path = p.as_ref().to_path_buf();

		let mut stringified = path.display().to_string();

		let extension = ".".to_owned() + Self::EXTENSION;

		if stringified.ends_with(&extension) {
			let range = unsafe { stringified.rfind(&extension).inner_unwrap().. };

			stringified.replace_range(range, "");

			Ok(stringified)
		} else {
			Err(FsError {
				kind: FsErrorType::InvalidFile { path },
				source: None,
			})
		}
	}

	/// The base directory of the Fs database.
	fn base_directory(&self) -> PathBuf;
}

#[cfg(feature = "fs")]
impl<RW: FsBackend> Backend for RW {
	type Error = FsError;

	fn init(&self) -> InitFuture<'_, FsError> {
		Box::pin(async move {
			if fs::read_dir(&self.base_directory()).await.is_err() {
				fs::create_dir_all(&self.base_directory())
					.await
					.map_err(FsError::io)?;
			}
			Ok(())
		})
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		Box::pin(async move {
			let result = fs::read_dir(self.resolve_path(&[table])).await;

			handle_io_result!(result, _val, Ok(true), Ok(false))
		})
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		Box::pin(async move {
			fs::create_dir(self.resolve_path(&[table]))
				.await
				.map_err(FsError::io)?;

			Ok(())
		})
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		Box::pin(async move {
			if self.has_table(table).await? {
				fs::remove_dir(self.resolve_path(&[table]))
					.await
					.map_err(FsError::io)?;
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
				fs::read_dir(self.resolve_path(&[table]))
					.await
					.map_err(FsError::io)?,
			);
			let mut output = Vec::new();

			while let Some(raw) = stream.next().await {
				let entry = raw.map_err(FsError::io)?;

				if entry.file_type().await.map_err(FsError::io)?.is_dir() {
					continue; // coverage:ignore-line
				}

				let filename = self.resolve_key(entry.file_name()).ok();

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
			let filename = RW::filename(id);
			let path = self.resolve_path(&[table, filename.as_str()]);
			handle_io_result!(fs::File::open(&path).await, file, {
				let reader = io::BufReader::<StdFile>::new(file.into_std().await);
				Ok(Some(self.from_reader(reader)?))
			})
		})
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		Box::pin(async move {
			let filename = RW::filename(id);
			let path = self.resolve_path(&[table, filename.as_str()]);
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
			let filepath = RW::filename(id);

			let path = self.resolve_path(&[table, filepath.as_str()]);

			if self.has(table, id).await? {
				return Err(FsError {
					kind: FsErrorType::FileAlreadyExists { path },
					source: None,
				});
			}

			let serialized = self.to_bytes(value)?;

			fs::write(path, serialized).await.map_err(FsError::io)?;

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
			let filepath = RW::filename(id);

			let path = self.resolve_path(&[table, filepath.as_str()]);

			fs::write(path, serialized).await.map_err(FsError::io)?;

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
			let filename = RW::filename(id);

			fs::remove_file(self.resolve_path(&[table, filename.as_str()]))
				.await
				.map_err(FsError::io)?;

			Ok(())
		})
	}
}

#[cfg(all(test, feature = "fs"))]
mod tests {
	use std::{io, path::PathBuf};

	use super::{FsBackend, FsError, FsErrorType};
	use crate::Entry;

	#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	struct MockFsBackend;

	#[cfg(not(tarpaulin_include))]
	impl FsBackend for MockFsBackend {
		const EXTENSION: &'static str = "test";

		fn from_reader<R, T>(&self, _: R) -> Result<T, FsError>
		where
			R: io::Read,
			T: Entry,
		{
			Err(FsError {
				kind: FsErrorType::Deserialization,
				source: None,
			})
		}

		fn to_bytes<T>(&self, _: &T) -> Result<Vec<u8>, FsError>
		where
			T: Entry,
		{
			Err(FsError {
				kind: FsErrorType::Serialization,
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
		let backend = MockFsBackend;

		let key = "foo.test";

		assert_eq!(backend.resolve_key(key)?, "foo");

		let invalid = "bar.json";

		assert!(backend.resolve_key(invalid).is_err());

		Ok(())
	}
}
