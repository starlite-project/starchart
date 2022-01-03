//! Helpers for creating file-system based backends.

use std::{
	fmt::Debug,
	fs::File as StdFile,
	io::{self, Read},
	iter::FromIterator,
	path::{Path, PathBuf},
};

use thiserror::Error;
use tokio::fs;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

use super::{
	future::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
		HasFuture, HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
	},
	Backend,
};
use crate::{util::InnerUnwrap, Entry};

/// An error occurred from an [`FsBackend`].
#[derive(Debug, Error)]
#[cfg(feature = "fs")]
pub enum FsError {
	/// An IO error occurred.
	#[error(transparent)]
	Io(#[from] io::Error),
	/// The path provided was not a directory.
	#[error("path {0} is not a directory")]
	PathNotDirectory(PathBuf),
	/// A serde error occurred.
	#[error("a serialization error occurred")]
	Serde,
	/// A file was found to be invalid.
	#[error("file {} is invalid", .0.display())]
	InvalidFile(PathBuf),
	/// A file already exists.
	#[error("file {} already exists", .0.display())]
	FileAlreadyExists(PathBuf),
}

macro_rules! handle_io_result {
	($res:expr, $name:ident, $okay:expr) => {
		handle_io_result!($res, $name, $okay, Ok(None))
	};
	($res:expr, $name:ident, $okay:expr, $not_found:expr) => {
		match $res {
			Ok($name) => $okay,
			Err(err) if err.kind() == ::std::io::ErrorKind::NotFound => $not_found,
			Err(e) => Err($crate::error::FsError::from(e)),
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
	/// Implementors should return a [`FsError::Serde`] error upon failure.
	fn from_reader<R, T>(rdr: R) -> Result<T, FsError>
	where
		R: Read,
		T: Entry;

	/// Turn a [`Entry`] into a writable [`Vec`].
	///
	/// # Errors
	///
	/// Implementors should return a [`FsError::Serde`] error upon failure.
	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
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
	/// Returns a [`FsError::InvalidFile`] when the file path does not have the correct extension.
	fn resolve_key<P: AsRef<Path>>(&self, p: P) -> Result<String, FsError> {
		let path = p.as_ref().to_path_buf();

		let mut stringified = path.display().to_string();

		let extension = ".".to_owned() + Self::EXTENSION;

		if stringified.ends_with(&extension) {
			let range = unsafe { stringified.rfind(&extension).inner_unwrap().. };

			stringified.replace_range(range, "");

			Ok(stringified)
		} else {
			Err(FsError::InvalidFile(path)) // coverage:ignore-line
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
				fs::create_dir_all(&self.base_directory()).await?;
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
			fs::create_dir(self.resolve_path(&[table])).await?;

			Ok(())
		})
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		Box::pin(async move {
			if self.has_table(table).await? {
				fs::remove_dir(self.resolve_path(&[table])).await?;
			}

			Ok(())
		})
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		Box::pin(async move {
			let mut stream = ReadDirStream::new(fs::read_dir(self.resolve_path(&[table])).await?);
			let mut output = Vec::new();

			while let Some(raw) = stream.next().await {
				let entry = raw?;

				if entry.file_type().await?.is_dir() {
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
				Ok(Some(RW::from_reader(reader)?))
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
				return Err(FsError::FileAlreadyExists(path));
			}

			let serialized = RW::to_bytes(value)?;

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
			let serialized = RW::to_bytes(value)?;
			let filepath = RW::filename(id);

			let path = self.resolve_path(&[table, filepath.as_str()]);

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
			let filename = RW::filename(id);

			fs::remove_file(self.resolve_path(&[table, filename.as_str()])).await?;

			Ok(())
		})
	}
}

#[cfg(all(test, feature = "fs"))]
mod tests {
	use std::{io, path::PathBuf};

	use super::{FsBackend, FsError};
	use crate::Entry;

	#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	struct MockFsBackend;

	#[cfg(not(tarpaulin_include))]
	impl FsBackend for MockFsBackend {
		const EXTENSION: &'static str = "test";

		fn from_reader<R, T>(_: R) -> Result<T, FsError>
		where
			R: io::Read,
			T: Entry,
		{
			Err(FsError::Serde)
		}

		fn to_bytes<T>(_: &T) -> Result<Vec<u8>, FsError>
		where
			T: Entry,
		{
			Err(FsError::Serde)
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
