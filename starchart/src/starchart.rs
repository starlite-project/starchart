//! The base structure to use for starchart.

use std::{ops::Deref, sync::Arc};

use futures_executor::block_on;

use crate::{atomics::Guard, backend::Backend};

/// The base structure for managing data.
///
/// The inner data is wrapped in an [`Arc`], so cloning
/// is cheap and will allow multiple accesses to the data.
#[derive(Debug, Default)]
pub struct Starchart<B: Backend> {
	backend: Arc<B>,
	pub(crate) guard: Arc<Guard>,
}

impl<B: Backend> Starchart<B> {
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
}

impl<B: Backend> Clone for Starchart<B> {
	fn clone(&self) -> Self {
		Self {
			backend: self.backend.clone(),
			guard: self.guard.clone(),
		}
	}
}

impl<B: Backend> Deref for Starchart<B> {
	type Target = B;

	fn deref(&self) -> &Self::Target {
		&*self.backend
	}
}

impl<B: Backend> Drop for Starchart<B> {
	fn drop(&mut self) {
		block_on(unsafe { self.backend.shutdown() });
	}
}
