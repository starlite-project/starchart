use std::{
	ffi::OsString,
	fmt::{Debug, Formatter, Result as FmtResult},
	fs::File as StdFile,
	io::{self, Read, Write},
	iter::FromIterator,
	marker::PhantomData,
	path::{Path, PathBuf},
};

use erased_serde::Serialize;
use tempfile::{NamedTempFile, PersistError};
use thiserror::Error;
use tokio::fs;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

use super::{
	future::{
		CreateFuture, CreateTableFuture, DeleteFuture, GetFuture, GetKeysFuture, HasFuture,
		HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
	},
	Backend,
};
use crate::{util::InnerUnwrap, Entry};

#[derive(Debug, Error)]
pub enum FsError {
	#[error(transparent)]
	Io(#[from] io::Error),
	#[error("path {0} is not a directory")]
	PathNotDirectory(PathBuf),
	#[error("a serialization error occurred")]
	Serde,
	#[error("file {} is invalid", .0.display())]
	InvalidFile(PathBuf),
	#[error("file {} already exists", .0.display())]
	FileAlreadyExists(PathBuf),
}

impl From<PersistError> for FsError {
	fn from(err: PersistError) -> Self {
		Self::Io(err.error)
	}
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

pub struct FsBackend<RW: FsReadWrite> {
	pub base_directory: PathBuf,
	inner: PhantomData<RW>,
}

impl<RW: FsReadWrite> FsBackend<RW> {
	pub fn new<P: AsRef<Path>>(base_directory: P, read_writer: RW) -> Result<Self, FsError> {
		let path = base_directory.as_ref().to_path_buf();

		if path.is_file() {
			Err(FsError::PathNotDirectory(path))
		} else {
			Ok(Self {
				base_directory: path,
				inner: PhantomData,
			})
		}
	}

	pub fn resolve_path<P: AsRef<Path>>(&self, path: &[P]) -> PathBuf {
		let mut base = self.base_directory.clone();

		for value in path {
			base = base.join(value);
		}

		base // coverage:ignore-line
	}

	pub fn resolve_key<P: AsRef<Path>>(&self, p: P) -> Result<String, FsError> {
		let path = p.as_ref().to_path_buf();

		let mut stringified = path.display().to_string();

		let extension = ".".to_string() + RW::EXTENSION;

		if stringified
			.rsplit('.')
			.next()
			.map(|ext| ext.eq_ignore_ascii_case(&extension))
			== Some(true)
		{
			let range = unsafe { stringified.rfind(&extension).inner_unwrap().. };

			stringified.replace_range(range, "");

			Ok(stringified)
		} else {
			Err(FsError::InvalidFile(path))
		}
	}
}

impl<RW: FsReadWrite> Debug for FsBackend<RW> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("FsBackend")
			.field("base_directory", &self.base_directory)
			.finish()
	}
}

impl<RW: FsReadWrite> Clone for FsBackend<RW> {
	fn clone(&self) -> Self {
		Self {
			base_directory: self.base_directory.clone(),
			inner: PhantomData,
		}
	}
}

impl<RW: FsReadWrite> Default for FsBackend<RW> {
	fn default() -> Self {
		Self {
			inner: PhantomData,
			base_directory: PathBuf::default(),
		}
	}
}

pub trait FsReadWrite: Copy + Send + Sync {
	const EXTENSION: &'static str;

	fn from_reader<R, T>(rdr: R) -> Result<T, FsError>
	where
		R: Read,
		T: Entry;

	fn to_bytes<T>(value: &T) -> Result<Vec<u8>, FsError>
	where
		T: Entry;

	fn filename(id: &str) -> String {
		id.to_string() + "." + Self::EXTENSION
	}
}

impl<RW: FsReadWrite> Backend for FsBackend<RW> {
	type Error = FsError;

	fn init(&self) -> InitFuture<'_, FsError> {
		Box::pin(async move {
			if fs::read_dir(&self.base_directory).await.is_err() {
				fs::create_dir_all(&self.base_directory).await?;
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

	fn delete_table<'a>(
		&'a self,
		table: &'a str,
	) -> super::future::DeleteTableFuture<'a, Self::Error> {
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

			let mut file = NamedTempFile::new()?;

			file.write_all(&serialized)?;

			file.persist(&path)?;

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

			let mut file = NamedTempFile::new()?;

			file.write_all(&serialized)?;

			file.persist(&path)?;

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
