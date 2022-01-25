#![allow(missing_docs, clippy::missing_panics_doc, clippy::missing_errors_doc)]
#[cfg(feature = "metadata")]
use std::any::type_name;
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	iter::FromIterator,
};

#[cfg(not(feature = "metadata"))]
use futures_util::{future::ok, Future};

#[cfg(feature = "metadata")]
use crate::METADATA_KEY;
use crate::{atomics::Guard, backend::Backend, util::is_metadata, Entry, IndexEntry, Key};

mod error;
#[doc(hidden)]
pub use self::error::{AccessorError, AccessorErrorType};

/// A chart accessor used for accessor access.
#[cfg(feature = "accessor")]
#[must_use = "a Accessor does nothing if not used."]
pub struct Accessor<'a, B> {
	backend: &'a B,
	guard: &'a Guard,
}

impl<'a, B> Accessor<'a, B> {
	pub(crate) const fn new(backend: &'a B, guard: &'a Guard) -> Self {
		Self { backend, guard }
	}
}

impl<'a, B: Backend> Accessor<'a, B> {
	pub async fn read_entry<S: Entry>(
		self,
		table: &str,
		key: &str,
	) -> Result<Option<S>, AccessorError> {
		// check the keys before we lock.
		Self::validate_metadata(table)?;
		Self::validate_metadata(key)?;

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
		Self::validate_metadata(table)?;

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

		let keys = keys
			.iter()
			.filter_map(|v| {
				if is_metadata(v) {
					None
				} else {
					Some(v.as_str())
				}
			})
			.collect::<Vec<_>>();

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
		Self::validate_metadata(table)?;
		Self::validate_metadata(key)?;

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
		Self::validate_metadata(table)?;

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
		Self::validate_metadata(table)?;
		Self::validate_metadata(key)?;

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
		Self::validate_metadata(table)?;
		Self::validate_metadata(key)?;

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
		Self::validate_metadata(table)?;

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

	#[cfg(feature = "metadata")]
	fn validate_metadata(key: &str) -> Result<(), AccessorError> {
		if key == METADATA_KEY {
			return Err(AccessorError {
				source: None,
				kind: AccessorErrorType::Metadata {
					type_and_table: None,
				},
			});
		}

		Ok(())
	}

	#[cfg(not(feature = "metadata"))]
	fn validate_metadata(_: &str) -> Result<(), AccessorError> {
		Ok(())
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

#[cfg(no_debug_non_exhaustive)]
impl<'a, B: Backend + Debug> Debug for Accessor<'a, B> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Accessor")
			.field("backend", &self.backend)
			.finish()
	}
}

#[cfg(not(no_debug_non_exhaustive))]
impl<'a, B: Backend + Debug> Debug for Accessor<'a, B> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Accessor")
			.field("backend", &self.backend)
			.finish_non_exhaustive()
	}
}

impl<'a, B: Backend> Clone for Accessor<'a, B> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, B: Backend> Copy for Accessor<'a, B> {}
