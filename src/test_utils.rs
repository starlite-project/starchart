#![allow(clippy::undocumented_unsafe_blocks)]

use std::{
	ffi::OsStr,
	fs,
	io::Result as IoResult,
	iter::FromIterator,
	path::{Path, PathBuf},
	sync::atomic::{AtomicBool, Ordering},
};

use thiserror::Error;

use crate::{
	backend::{
		future::{
			CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture,
			GetKeysFuture, HasFuture, HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
		},
		Backend, CacheBackend, CacheError,
	},
	Entry,
};

#[derive(Debug, Clone)]
pub struct FsCleanup(PathBuf);

impl FsCleanup {
	pub fn new(test_name: &str, should_create: bool) -> IoResult<Self> {
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.join("target")
			.join("tests")
			.join(test_name);

		if should_create {
			fs::create_dir_all(&path)?;
		}

		Ok(Self(path))
	}
}

impl AsRef<Path> for FsCleanup {
	fn as_ref(&self) -> &Path {
		self.0.as_ref()
	}
}

impl AsRef<OsStr> for FsCleanup {
	fn as_ref(&self) -> &OsStr {
		self.0.as_ref()
	}
}

impl Drop for FsCleanup {
	#[allow(clippy::let_underscore_drop)]
	fn drop(&mut self) {
		let _ = fs::remove_dir_all(&self.0);
	}
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct MockBackendError(#[from] CacheError);

#[derive(Debug, Default)]
pub struct MockBackend {
	inner: CacheBackend,
	initialized: AtomicBool,
}

impl MockBackend {
	pub fn new() -> Self {
		Self {
			inner: CacheBackend::new(),
			initialized: AtomicBool::new(false),
		}
	}

	pub fn is_initialized(&self) -> bool {
		self.initialized.load(Ordering::SeqCst)
	}
}

impl Backend for MockBackend {
	type Error = MockBackendError;

	fn init(&self) -> InitFuture<'_, Self::Error> {
		Box::pin(async move {
			self.initialized.store(true, Ordering::SeqCst);
			Ok(())
		})
	}

	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
		Box::pin(async move { Ok(self.inner.has_table(table).await?) })
	}

	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
		Box::pin(async move { Ok(self.inner.create_table(table).await?) })
	}

	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
		Box::pin(async move { Ok(self.inner.delete_table(table).await?) })
	}

	fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
	where
		I: FromIterator<String>,
	{
		Box::pin(async move { Ok(self.inner.get_keys(table).await?) })
	}

	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry,
	{
		Box::pin(async move { Ok(self.inner.get(table, id).await?) })
	}

	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
		Box::pin(async move { Ok(self.inner.has(table, id).await?) })
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
		Box::pin(async move { Ok(self.inner.create(table, id, value).await?) })
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
		Box::pin(async move { Ok(self.inner.update(table, id, value).await?) })
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
		Box::pin(async move { Ok(self.inner.replace(table, id, value).await?) })
	}

	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
		Box::pin(async move { Ok(self.inner.delete(table, id).await?) })
	}
}
