mod error;

use std::borrow::Cow;

#[doc(hidden)]
pub use self::error::{ActionError, ActionErrorType, MissingValue};
use crate::{backend::Backend, util::InnerUnwrap, Entry, IndexEntry, Key, Starchart};

#[derive(Debug)]
pub struct Action<'v, D: ?Sized> {
	table: &'v str,
	key: Option<Cow<'static, str>>,
	data: Option<&'v D>,
}

impl<'v, D: ?Sized> Action<'v, D> {
	pub const fn new(table: &'v str) -> Self {
		Self {
			table,
			key: None,
			data: None,
		}
	}

	pub const fn table(&self) -> &str {
		self.table
	}

	pub fn key(&self) -> Option<&str> {
		self.key.as_deref()
	}
}

impl<'v, D: Entry + ?Sized> Action<'v, D> {
	pub fn data(&self) -> Option<&'v D> {
		self.data
	}

	#[must_use]
	pub fn entry(&self) -> Option<(&str, &'v D)> {
		match (self.key(), self.data()) {
			(Some(k), Some(v)) => Some((k, v)),
			_ => None,
		}
	}

	pub fn with_key<K: Key>(table: &'v str, key: &K) -> Self {
		let mut act = Self::new(table);

		act.set_key(key);

		act
	}

	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.key.replace(key.to_key());

		self
	}

	pub fn set_data(&mut self, entry: &'v D) -> &mut Self {
		self.data.replace(entry);

		self
	}

	// run methods

	pub async fn create_entry<B: Backend>(
		mut self,
		chart: &Starchart<B>,
	) -> Result<(), ActionError> {
		self.validate_entry()?;
		self.validate_table()?;

		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let (table, key, entry) = unsafe {
			(
				self.table,
				self.key.take().inner_unwrap(),
				self.data.take().inner_unwrap(),
			)
		};

		self.check_table(backend).await?;
		self.check_metadata(backend).await?;

		backend
			.ensure(table, &key, &*entry)
			.await
			.map_err(|e| ActionError {
				source: Some(Box::new(e)),
				kind: ActionErrorType::Backend,
			})?;

		drop(lock);

		Ok(())
	}

	pub async fn read_entry<B: Backend>(
		mut self,
		chart: &Starchart<B>,
	) -> Result<Option<D>, ActionError> {
		self.validate_data()?;
		self.validate_key()?;

		let lock = chart.guard.shared().await;

		let backend = &**chart;

		let (table, key) = unsafe { (self.table, self.key.take().inner_unwrap()) };

		self.check_table(backend).await?;
		self.check_metadata(backend).await?;

		let res = backend.get(table, &key).await.map_err(|e| ActionError {
			source: Some(Box::new(e)),
			kind: ActionErrorType::Backend,
		})?;

		drop(lock);

		Ok(res)
	}

	#[cfg(feature = "metadata")]
	async fn check_metadata<B: Backend>(&self, backend: &B) -> Result<(), ActionError> {
		backend
			.get::<D>(self.table, crate::METADATA_KEY)
			.await
			.map(|_| {})
			.map_err(|e| ActionError {
				source: Some(Box::new(e)),
				kind: ActionErrorType::Metadata(Some(self.table.to_owned())),
			})
	}

	#[cfg(not(feature = "metadata"))]
	fn check_metadata<B: Backend>(
		&self,
		_: &B,
	) -> impl futures_util::Future<Output = Result<(), ActionError>> {
		futures_util::future::ok(())
	}

	async fn check_table<B: Backend>(&self, backend: &B) -> Result<(), ActionError> {
		if backend
			.has_table(self.table)
			.await
			.map_err(|e| ActionError {
				source: Some(Box::new(e)),
				kind: ActionErrorType::Backend,
			})? {
			Ok(())
		} else {
			Err(ActionError {
				source: None,
				kind: ActionErrorType::SomethingMissing(MissingValue::Table),
			})
		}
	}

	#[cfg(feature = "metadata")]
	#[allow(clippy::unused_self)]
	fn validate_metadata(&self, key: Option<&str>) -> Result<(), ActionError> {
		if key == Some(crate::METADATA_KEY) {
			return Err(ActionError {
				source: None,
				kind: ActionErrorType::Metadata(None),
			});
		}

		Ok(())
	}

	#[cfg(not(feature = "metadata"))]
	#[allow(clippy::unused_self)]
	fn validate_metadata(&self, _: Option<&str>) -> Result<(), ActionError> {
		Ok(())
	}

	fn validate_key(&self) -> Result<(), ActionError> {
		if self.key.is_none() {
			return Err(ActionError {
				source: None,
				kind: ActionErrorType::SomethingMissing(MissingValue::Key),
			});
		}

		self.validate_metadata(self.key.as_deref())
	}

	fn validate_data(&self) -> Result<(), ActionError> {
		if self.data.is_none() {
			return Err(ActionError {
				source: None,
				kind: ActionErrorType::SomethingMissing(MissingValue::Data),
			});
		}

		Ok(())
	}

	fn validate_table(&self) -> Result<(), ActionError> {
		self.validate_metadata(Some(self.table))
	}

	fn validate_entry(&self) -> Result<(), ActionError> {
		self.validate_key()?;
		self.validate_data()
	}
}

impl<'v, D: IndexEntry + ?Sized> Action<'v, D> {
	pub fn with_entry(table: &'v str, entry: &'v D) -> Self {
		let mut act = Self::new(table);

		act.set_entry(entry);

		act
	}

	pub fn set_entry(&mut self, entry: &'v D) -> &mut Self {
		self.set_key(entry.key()).set_data(entry)
	}
}

impl<'v, D: ?Sized> Clone for Action<'v, D> {
	fn clone(&self) -> Self {
		Self {
			table: self.table,
			key: self.key.clone(),
			data: self.data,
		}
	}
}
