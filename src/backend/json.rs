use std::{
	ffi::OsString,
	io::{self, ErrorKind},
	iter::FromIterator,
	path::{Path, PathBuf},
};

use futures_util::StreamExt;
use thiserror::Error;
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

use super::{
	future::{
		CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture, GetKeysFuture,
		HasFuture, HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
	},
	Backend,
};
use crate::Entry;

/// An error returned from the [`JsonBackend`].
///
/// [`JsonBackend`]: crate::backend::JsonBackend
#[doc(cfg(feature = "json"))]
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JsonError {
	/// The path provided was not a directory.
	#[error("path {0} is not a directory")]
	PathNotDirectory(PathBuf),
	/// An IO error occurred.
	#[error("an IO error occurred: {0}")]
	Io(#[from] io::Error),
	/// An error occurred serializing data.
	#[error("a JSON error occurred")]
	SerdeJson(#[from] serde_json::Error),
	/// A file was found to be invalid.
	#[error("file {} is invalid", .0.display())]
	InvalidFile(PathBuf),
	/// The file already exists
	#[error("file {} already exists", .0.display())]
	FileAlreadyExists(PathBuf),
}

/// A JSON based backend, uses [`serde_json`] to read and write files.
#[doc(cfg(feature = "json"))]
#[derive(Debug, Default, Clone)]
pub struct JsonBackend {
	base_directory: PathBuf,
}

impl JsonBackend {
	/// Creates a new [`JsonBackend`].
	///
	/// # Errors
	///
	/// Returns a [`JsonError::Io`] if the path appears to be a file.
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
		let path = path.as_ref().to_path_buf();

		if path.is_file() {
			Err(JsonError::PathNotDirectory(path))
		} else {
			Ok(Self {
				base_directory: path,
			})
		}
	}

	fn resolve_path<P: AsRef<Path>>(&self, path: &[P]) -> PathBuf {
		let mut base = self.base_directory.clone();

		for value in path {
			base = base.join(value);
		}

		base
	}

	fn resolve_key(file: OsString) -> Result<String, JsonError> {
		let path: PathBuf = file.into();

		let mut stringified = path.display().to_string();

		if stringified
			.rsplit('.')
			.next()
			.map(|ext| ext.eq_ignore_ascii_case("json"))
			== Some(true)
		{
			let range = unsafe { stringified.rfind(".json").unwrap_unchecked().. };

			stringified.replace_range(range, "");

			Ok(stringified)
		} else {
			Err(JsonError::InvalidFile(path))
		}
	}
}

impl Backend for JsonBackend {
	type Error = JsonError;

	fn init(&self) -> InitFuture<'_, JsonError> {
		Box::pin(async move {
			if fs::read_dir(&self.base_directory).await.is_err() {
				fs::create_dir_all(&self.base_directory).await?;
			}

			Ok(())
		})
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, JsonError> {
		Box::pin(async move {
			let result = fs::read_dir(self.resolve_path(&[table])).await;

			match result {
				Ok(_) => Ok(true),
				Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
				Err(e) => Err(e.into()),
			}
		})
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, JsonError> {
		Box::pin(async move {
			fs::create_dir(self.resolve_path(&[table])).await?;

			Ok(())
		})
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, JsonError> {
		Box::pin(async move {
			if self.has_table(table).await? {
				fs::remove_dir_all(self.resolve_path(&[table])).await?;
			}

			Ok(())
		})
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, JsonError>
	where
		I: FromIterator<String>,
	{
		Box::pin(async move {
			let mut stream = ReadDirStream::new(fs::read_dir(self.resolve_path(&[table])).await?);
			let mut output = Vec::new();

			while let Some(raw) = stream.next().await {
				let entry = raw?;

				if entry.file_type().await?.is_dir() {
					continue;
				}

				let filename = Self::resolve_key(entry.file_name()).ok();

				if filename.is_none() {
					continue;
				}

				output.push(unsafe { filename.unwrap_unchecked() });
			}

			Ok(output.into_iter().collect())
		})
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, JsonError>
	where
		D: Entry,
	{
		Box::pin(async move {
			let filename = id.to_owned() + ".json";
			let path = self.resolve_path(&[table, filename.as_str()]);
			let file: std::fs::File = {
				let res = fs::File::open(path).await;
				match res {
					Ok(file) => file.into_std().await,
					Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
					Err(e) => return Err(e.into()),
				}
			};
			let reader = io::BufReader::new(file);
			Ok(Some(serde_json::from_reader(reader)?))
		})
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, JsonError> {
		Box::pin(async move {
			let filename = id.to_owned() + ".json";
			let file = fs::read(self.resolve_path(&[table, filename.as_str()])).await;

			match file {
				Ok(_) => Ok(true),
				Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
				Err(e) => Err(e.into()),
			}
		})
	}

	fn create<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> CreateFuture<'a, JsonError>
	where
		S: Entry,
	{
		Box::pin(async move {
			let filepath = id.to_owned() + ".json";

			let path = self.resolve_path(&[table, filepath.as_str()]);

			if self.has(table, id).await? {
				return Err(JsonError::FileAlreadyExists(path));
			}

			let serialized = serde_json::to_string(value)?.into_bytes();

			fs::write(path, serialized).await?;

			Ok(())
		})
	}

	fn update<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> UpdateFuture<'a, JsonError>
	where
		S: Entry,
	{
		Box::pin(async move {
			let serialized = serde_json::to_string(value)?.into_bytes();

			let filepath = id.to_owned() + ".json";

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
	) -> ReplaceFuture<'a, JsonError>
	where
		S: Entry,
	{
		Box::pin(async move {
			self.update(table, id, value).await?;

			Ok(())
		})
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, JsonError> {
		Box::pin(async move {
			let filename = id.to_owned() + ".json";

			fs::remove_file(self.resolve_path(&[table, filename.as_str()])).await?;

			Ok(())
		})
	}
}

#[cfg(all(test, feature = "json"))]
mod tests {
	use std::{
		fmt::Debug,
		path::{Path, PathBuf},
	};

	use static_assertions::assert_impl_all;

	use crate::backend::{Backend, JsonBackend, JsonError};

	assert_impl_all!(JsonBackend: Backend, Clone, Debug, Default, Send, Sync);

	fn create_test_path(test_name: &str) -> String {
		let path = Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("target")
			.join("tests")
			.join(test_name);

		path.to_str().unwrap().to_owned()
	}

	#[test]
	fn new() -> Result<(), JsonError> {
		let path = create_test_path("new");
		let backend = JsonBackend::new(&path)?;

		assert_eq!(backend.base_directory, PathBuf::from(path));

		Ok(())
	}

	#[test]
	fn resolve_path() -> Result<(), JsonError> {
		let path = create_test_path("resolve_path");
		let backend = JsonBackend::new(&path)?;

		assert_eq!(
			backend.resolve_path(&["table", "id.json"]),
			PathBuf::from(path).join("table/id.json")
		);

		Ok(())
	}

	#[test]
	fn resolve_key() -> Result<(), JsonError> {
		let path = PathBuf::new().join("foo.json");

		assert_eq!(JsonBackend::resolve_key(path.into())?, "foo");

		Ok(())
	}
}
