//! The base structure to use for starchart.

use std::sync::Arc;

use futures::executor::block_on;
use parking_lot::RwLock;

use crate::{
	action::{ActionRunner, ActionValidationError},
	backend::Backend,
};

/// The base structure for managing data.
///
/// The inner data is wrapped in an [`Arc`], so cloning
/// is cheap and will allow multiple accesses to the data.
#[derive(Debug, Default)]
pub struct Starchart<B: Backend> {
	backend: Arc<B>,
	pub(crate) guard: Arc<RwLock<()>>,
}

impl<B: Backend> Starchart<B> {
	/// Gives access to the raw [`Backend`] instance.
	///
	/// # Safety
	///
	/// Accessing the backend functions directly isn't inheritly unsafe, however
	/// care must be taken to ensure the data isn't modified directly, and
	/// that [`Backend::shutdown`] isn't directly called.
	#[must_use]
	pub unsafe fn backend(&self) -> &B {
		&*self.backend
	}

	/// Creates a new [`Starchart`], and initializes the [`Backend`].
	///
	/// # Errors
	///
	/// Any errors that [`Backend::init`] can raise.
	pub async fn new(backend: B) -> Result<Self, B::Error> {
		backend.init().await?;
		Ok(Self {
			backend: Arc::new(backend),
			guard: Arc::default(),
		})
	}

	/// Runs an [`Action`], returning whatever the possible [`ActionRunner`] implementation is declared as.
	///
	/// # Errors
	///
	/// Anything that the [`Action`] could return while running.
	///
	/// [`Action`]: crate::action::Action
	pub async fn run<Success, Failure>(
		&self,
		action: impl ActionRunner<B, Success, Failure>,
	) -> Result<Result<Success, Failure>, ActionValidationError>
	where
		Failure: From<<B as Backend>::Error>,
	{
		unsafe {
			action.validate()?;
			Ok(action.run(self).await)
		}
	}
}

impl<B: Backend> Clone for Starchart<B> {
	fn clone(&self) -> Self {
		Self {
			backend: self.backend.clone(),
			guard: self.guard.clone(),
		}
	}
}

impl<B: Backend> Drop for Starchart<B> {
	fn drop(&mut self) {
		block_on(unsafe { self.backend.shutdown() });
	}
}

#[cfg(all(test, feature = "cache"))]
mod tests {
	use std::{
		fmt::Debug,
		iter::FromIterator,
		sync::{
			atomic::{AtomicBool, Ordering},
			Arc,
		},
	};

	use static_assertions::assert_impl_all;
	use thiserror::Error;

	use crate::{
		backend::{
			future::{
				CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, GetFuture,
				GetKeysFuture, HasFuture, HasTableFuture, InitFuture, ReplaceFuture, UpdateFuture,
			},
			Backend, CacheBackend,
		},
		error::CacheError,
		Entry, Starchart,
	};

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

		#[cfg(not(tarpaulin_include))]
		fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error> {
			Box::pin(async move { Ok(self.inner.has_table(table).await?) })
		}

		#[cfg(not(tarpaulin_include))]
		fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error> {
			Box::pin(async move { Ok(self.inner.create_table(table).await?) })
		}

		#[cfg(not(tarpaulin_include))]
		fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error> {
			Box::pin(async move { Ok(self.inner.delete_table(table).await?) })
		}

		#[cfg(not(tarpaulin_include))]
		fn get_keys<'a, I>(&'a self, table: &'a str) -> GetKeysFuture<'a, I, Self::Error>
		where
			I: FromIterator<String>,
		{
			Box::pin(async move { Ok(self.inner.get_keys(table).await?) })
		}

		#[cfg(not(tarpaulin_include))]
		fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
		where
			D: Entry,
		{
			Box::pin(async move { Ok(self.inner.get(table, id).await?) })
		}

		#[cfg(not(tarpaulin_include))]
		fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error> {
			Box::pin(async move { Ok(self.inner.has(table, id).await?) })
		}

		#[cfg(not(tarpaulin_include))]
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

		#[cfg(not(tarpaulin_include))]
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

		#[cfg(not(tarpaulin_include))]
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

		#[cfg(not(tarpaulin_include))]
		fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error> {
			Box::pin(async move { Ok(self.inner.delete(table, id).await?) })
		}
	}

	assert_impl_all!(Starchart<MockBackend>: Clone, Debug, Default, Drop);

	#[tokio::test]
	#[cfg_attr(miri, ignore)]
	async fn new_and_drop() -> Result<(), MockBackendError> {
		let backend = MockBackend::new();
		let starchart = Starchart::new(backend).await?;

		// SAFETY: this is a test
		let backend = unsafe { starchart.backend() };

		assert!(backend.is_initialized());

		Ok(())
	}

	#[tokio::test]
	#[allow(clippy::redundant_clone)]
	#[cfg_attr(miri, ignore)]
	async fn clone() -> Result<(), MockBackendError> {
		let backend = MockBackend::new();
		let starchart = Starchart::new(backend).await?;

		{
			let cloned = starchart.clone();
			let cloned_backend = &cloned.backend;
			assert_eq!(Arc::strong_count(cloned_backend), 2);
		}

		Ok(())
	}
}