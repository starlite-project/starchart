#![allow(missing_docs)]
use std::{iter::FromIterator, pin::Pin};

use futures_util::{future::err, Future, FutureExt};

#[cfg(feature = "metadata")]
use crate::METADATA_KEY;
use crate::{
	atomics::Guard,
	backend::Backend,
	util::{is_metadata, InnerUnwrap},
	Entry, Starchart,
};

type AccessorFuture<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

mod error;
#[doc(hidden)]
pub use self::error::{AccessorError, AccessorErrorType};

/// A chart accessor used for passive access.
#[cfg(feature = "passive")]
#[must_use = "a ChartAccessor does nothing if not used."]
pub struct ChartAccessor<'a, B> {
	backend: &'a B,
	guard: &'a Guard,
}

impl<'a, B> ChartAccessor<'a, B> {
	pub(crate) const fn new(backend: &'a B, guard: &'a Guard) -> Self {
		Self { backend, guard }
	}
}

impl<'a, B: Backend> ChartAccessor<'a, B> {
	#[must_use]
	pub fn read_entry<S: Entry + 'static>(
		self,
		table: &'a str,
		key: &'a str,
	) -> AccessorFuture<'a, Result<Option<S>, AccessorError>> {
		// check the keys before we lock.
		if let Some(e) = Self::check_metadata(table).or_else(|| Self::check_metadata(key)) {
			return err(e).boxed();
		}

		let lock = self.guard.shared();
		let res = self
			.backend
			.get::<S>(table, key)
			.map(|res| {
				res.map_err(|e| AccessorError {
					source: Some(Box::new(e)),
					kind: AccessorErrorType::Backend,
				})
			})
			.boxed();
		drop(lock);

		res
	}

	#[must_use]
	pub fn read_table<S: Entry + 'static, I: FromIterator<S> + Send + 'static>(
		self,
		table: &'a str,
	) -> AccessorFuture<'a, Result<I, AccessorError>> {
		let lock = self.guard.shared();
		let res = Box::pin(async move {
			if let Some(e) = Self::check_metadata(table) {
				return Err(e);
			}

			let mut keys =
				self.backend
					.get_keys::<Vec<_>>(table)
					.await
					.map_err(|e| AccessorError {
						source: Some(Box::new(e)),
						kind: AccessorErrorType::Backend,
					})?;

			keys = keys.into_iter().filter(|k| !is_metadata(k)).collect();

			let borrowed_keys = keys.iter().map(String::as_str).collect::<Vec<_>>();

			self.backend
				.get_all::<S, I>(table, &borrowed_keys)
				.await
				.map_err(|e| AccessorError {
					source: Some(Box::new(e)),
					kind: AccessorErrorType::Backend,
				})
		});

		drop(lock);

		res
	}

	pub fn create_entry<S: Entry + 'static>(
		self,
		table: &'a str,
		key: &'a str,
		entry: &'a S,
	) -> AccessorFuture<'a, Result<(), AccessorError>> {
		let lock = self.guard.exclusive();
		let res = Box::pin(async move {
			if self.backend.has(table, key).await.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})? {
				return Ok(());
			}

			todo!()
		});

		drop(lock);

		res
	}

	// we return an option to make it compatible with the future above.
	#[cfg(feature = "metadata")]
	fn check_metadata(key: &str) -> Option<AccessorError> {
		if key == METADATA_KEY {
			return Some(AccessorError {
				source: None,
				kind: AccessorErrorType::Metadata {
					type_and_table: None,
				},
			});
		}

		None
	}

	#[cfg(not(feature = "metadata"))]
	fn check_metadata(key: &str) -> Option<AccessorError> {
		None
	}
}

impl<'a, B: Backend> Clone for ChartAccessor<'a, B> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, B: Backend> Copy for ChartAccessor<'a, B> {}
