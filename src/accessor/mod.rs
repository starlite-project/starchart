#![allow(missing_docs, clippy::missing_panics_doc, clippy::missing_errors_doc)]
#[cfg(feature = "metadata")]
use std::any::type_name;
use std::iter::FromIterator;

#[cfg(not(feature = "metadata"))]
use futures_util::{future::ok, Future};

#[cfg(feature = "metadata")]
use crate::METADATA_KEY;
use crate::{atomics::Guard, backend::Backend, Entry, IndexEntry, Key};

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
	pub async fn read_entry<S: Entry>(
		self,
		table: &str,
		key: &str,
	) -> Result<Option<S>, AccessorError> {
		// check the keys before we lock.
		if let Some(e) = Self::validate_metadata(table).or_else(|| Self::validate_metadata(key)) {
			return Err(e);
		}
		let lock = self.guard.shared();

		self.check_metadata::<S>(table).await?;
		let value = self
			.backend
			.get::<S>(table, key)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			});

		drop(lock);
		value
	}

	pub async fn read_table<S: Entry, I: FromIterator<S> + Send>(
		self,
		table: &str,
	) -> Result<I, AccessorError> {
		if let Some(e) = Self::validate_metadata(table) {
			return Err(e);
		}

		let lock = self.guard.shared();

		self.check_metadata::<S>(table).await?;

		let keys = self
			.backend
			.get_keys::<Vec<_>>(table)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		let values = self
			.backend
			.get_all(table, &keys)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			});

		drop(lock);

		values
	}

	pub async fn create_entry<S: Entry>(
		self,
		table: &str,
		key: &str,
		entry: &S,
	) -> Result<(), AccessorError> {
		if let Some(e) = Self::validate_metadata(table).or_else(|| Self::validate_metadata(key)) {
			return Err(e);
		}

		let lock = self.guard.exclusive();

		if self
			.backend
			.has(table, key)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})? {
			return Ok(());
		}

		self.check_metadata::<S>(table).await?;

		self.backend
			.create(table, key, entry)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		drop(lock);

		Ok(())
	}

	pub async fn create_index_entry<S: IndexEntry>(
		self,
		table: &str,
		entry: &S,
	) -> Result<(), AccessorError> {
		self.create_entry(table, entry.key().to_key().as_str(), entry)
			.await
	}

	pub async fn create_table<S: Entry>(self, table: &str) -> Result<(), AccessorError> {
		if let Some(e) = Self::validate_metadata(table) {
			return Err(e);
		}

		let lock = self.guard.exclusive();

		self.backend
			.create_table(table)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		self.create_metadata::<S>(table).await?;

		drop(lock);

		Ok(())
	}

	pub async fn update_entry<S: Entry>(
		self,
		table: &str,
		key: &str,
		entry: &S,
	) -> Result<(), AccessorError> {
		if let Some(e) = Self::validate_metadata(table).or_else(|| Self::validate_metadata(key)) {
			return Err(e);
		}

		let lock = self.guard.exclusive();

		self.check_metadata::<S>(table).await?;

		self.backend
			.update(table, key, entry)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		drop(lock);

		Ok(())
	}

	pub async fn update_index_entry<S: IndexEntry>(
		self,
		table: &str,
		entry: &S,
	) -> Result<(), AccessorError> {
		self.update_entry(table, entry.key().to_key().as_str(), entry)
			.await
	}

	pub async fn delete_entry<S: Entry>(
		self,
		table: &str,
		key: &str,
	) -> Result<bool, AccessorError> {
		if let Some(e) = Self::validate_metadata(table).or_else(|| Self::validate_metadata(key)) {
			return Err(e);
		}

		let lock = self.guard.exclusive();

		self.check_metadata::<S>(table).await?;

		let exists = self
			.backend
			.has(table, key)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		self.backend
			.delete(table, key)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		let new_exists = self
			.backend
			.has(table, key)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		drop(lock);

		Ok(exists != new_exists)
	}

	pub async fn delete_table<S: Entry>(self, table: &str) -> Result<bool, AccessorError> {
		if let Some(e) = Self::validate_metadata(table) {
			return Err(e);
		}

		let lock = self.guard.exclusive();

		self.check_metadata::<S>(table).await?;

		let exists = self
			.backend
			.has_table(table)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		self.backend
			.delete_table(table)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		let new_exists = self
			.backend
			.has_table(table)
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Backend,
			})?;

		drop(lock);

		Ok(exists != new_exists)
	}

	// we return an option to make it compatible with the future above.
	#[cfg(feature = "metadata")]
	fn validate_metadata(key: &str) -> Option<AccessorError> {
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
	fn validate_metadata(key: &str) -> Option<AccessorError> {
		None
	}

	#[cfg(feature = "metadata")]
	async fn check_metadata<S: Entry>(self, table_name: &str) -> Result<(), AccessorError> {
		self.backend
			.get::<S>(table_name, METADATA_KEY)
			.await
			.map(|_| {})
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Metadata {
					type_and_table: Some((type_name::<S>(), table_name.to_owned())),
				},
			})
	}

	#[cfg(not(feature = "metadata"))]
	fn check_metadata<S: Entry>(self, _: &str) -> impl Future<Output = Result<(), AccessorError>> {
		ok(())
	}

	#[cfg(feature = "metadata")]
	async fn create_metadata<S: Entry>(self, table_name: &str) -> Result<(), AccessorError> {
		self.backend
			.ensure(table_name, METADATA_KEY, &S::default())
			.await
			.map_err(|e| AccessorError {
				source: Some(Box::new(e)),
				kind: AccessorErrorType::Metadata {
					type_and_table: Some((type_name::<S>(), table_name.to_owned())),
				},
			})
	}

	#[cfg(not(feature = "metadata"))]
	fn create_metadata<S: Entry>(self, _: &str) -> impl Future<Output = Result<(), AccessorError>> {
		ok(())
	}
}

impl<'a, B: Backend> Clone for ChartAccessor<'a, B> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, B: Backend> Copy for ChartAccessor<'a, B> {}
