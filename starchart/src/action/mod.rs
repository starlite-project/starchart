//! The action structs for CRUD operations.

mod error;

use std::{borrow::Cow, iter::FromIterator};

#[doc(hidden)]
pub use self::error::{ActionError, ActionErrorType, MissingValue};
use crate::{backend::Backend, Entry, IndexEntry, Key, Starchart};

/// An [`Action`] for an easy [`CRUD`] operation with a [`Starchart`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
#[derive(Debug, PartialEq, Eq)]
#[must_use = "Actions do nothing on their own"]
pub struct Action<'v, D: ?Sized> {
	table: &'v str,
	key: Option<Cow<'static, str>>,
	data: Option<&'v D>,
}

impl<'v, D: ?Sized> Action<'v, D> {
	/// Creates a new [`Action`] with the specified table.
	///
	/// ```rust
	/// # use starchart::Action;
	/// # fn ignore_me() -> Action<'static, u8> {
	/// let act = Action::new("foo");
	///
	/// assert_eq!(act.table(), "foo");
	/// # act }
	/// ```
	pub const fn new(table: &'v str) -> Self {
		Self {
			table,
			key: None,
			data: None,
		}
	}

	/// Get a reference to the table.
	#[must_use = "getting Action information does nothing on it's own"]
	pub const fn table(&self) -> &str {
		self.table
	}

	/// Get a reference to the key, if one is set.
	#[must_use = "getting Action information does nothing on it's own"]
	pub fn key(&self) -> Option<&str> {
		self.key.as_deref()
	}
}

impl<'v, D: Entry + ?Sized> Action<'v, D> {
	/// Get a reference to the data, if any is set.
	#[must_use = "getting Action information does nothing on it's own"]
	pub fn data(&self) -> Option<&'v D> {
		self.data
	}

	/// Get a reference to the entry, if it is set.
	#[must_use = "getting Action information does nothing on it's own"]
	pub fn entry(&self) -> Option<(&str, &'v D)> {
		self.key().zip(self.data())
	}

	/// Creates a new [`Action`] with the specified [`Key`].
	pub fn with_key<K: Key>(table: &'v str, key: &K) -> Self {
		let mut act = Self::new(table);

		act.set_key(key);

		act
	}

	/// Sets a [`Key`] on an [`Action`].
	///
	/// ```rust
	/// # use starchart::Action;
	/// # fn ignore_me() -> Action<'static, u8> {
	/// let mut act = Action::new("foo");
	///
	/// assert_eq!(act.key(), None);
	///
	/// act.set_key(&"bar"); // need a borrowed type.
	///
	/// assert_eq!(act.key(), Some("bar"));
	/// # act }
	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.key.replace(key.to_key());

		self
	}

	/// Sets the [`Entry`] for this [`Action`].
	///
	/// ```rust
	/// # use starchart::Action;
	/// # use serde::{Serialize, Deserialize};
	/// # #[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
	/// struct Settings { // our entry
	///     key: String,
	/// }
	///
	/// let settings = Settings {
	///     key: "John".to_owned(),
	/// };
	///
	/// let mut act = Action::new("foo");
	///
	/// assert_eq!(act.data(), None);
	///
	/// act.set_data(&settings);
	///
	/// assert_eq!(act.data(), Some(&settings));
	/// ```
	pub fn set_data(&mut self, entry: &'v D) -> &mut Self {
		self.data.replace(entry);

		self
	}

	// run methods

	/// Creates a new entry in the [`Starchart`] with the data specified in the action.
	///
	/// # Errors
	///
	/// This raises an error if the key or data isn't set (with the [`set_key`] and [`set_data`] methods, or [`set_entry`]), or if the [`Backend`] encounters an error.
	///
	/// [`set_key`]: Self::set_key
	/// [`set_data`]: Self::set_data
	/// [`set_entry`]: Self::set_entry
	pub async fn create_entry<B: Backend>(self, chart: &Starchart<B>) -> Result<(), ActionError> {
		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let table = self.validate_table()?;
		let (key, entry) = self.validate_entry()?;

		self.check_table(backend).await?;
		self.check_metadata(backend).await?;

		backend
			.ensure(table, key, &*entry)
			.await
			.map_err(ActionError::from_backend)?;

		drop(lock);

		Ok(())
	}

	/// Reads an entry from the [`Starchart`].
	///
	/// # Errors
	///
	/// This raises an error if the key isn't set (with the [`set_key`] method), or if the [`Backend`] encounters an error.
	///
	/// [`set_key`]: Self::set_key
	pub async fn read_entry<B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<Option<D>, ActionError> {
		let lock = chart.guard.shared().await;

		let backend = &**chart;

		let table = self.validate_table()?;
		let key = self.validate_key()?;

		self.check_table(backend).await?;
		self.check_metadata(backend).await?;

		let res = backend
			.get(table, key)
			.await
			.map_err(ActionError::from_backend)?;

		drop(lock);

		Ok(res)
	}

	/// Updates an entry within the [`Starchart`].
	///
	/// # Errors
	///
	/// This raises an error if the key or data isn't set (with the [`set_key`] and [`set_data`] methods, or [`set_entry`]), or if the [`Backend`] encounters an error.
	///
	/// [`set_key`]: Self::set_key
	/// [`set_data`]: Self::set_data
	/// [`set_entry`]: Self::set_entry
	pub async fn update_entry<B: Backend>(self, chart: &Starchart<B>) -> Result<(), ActionError> {
		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let table = self.validate_table()?;
		let (key, entry) = self.validate_entry()?;

		self.check_table(backend).await?;
		self.check_metadata(backend).await?;

		backend
			.update(table, key, entry)
			.await
			.map_err(ActionError::from_backend)?;

		drop(lock);

		Ok(())
	}

	/// Deletes an entry from the [`Starchart`], returning whether or not the item was deleted.
	///
	/// # Errors
	///
	/// This raises an error if the key isn't set (with the [`set_key`] method), or if the [`Backend`] encounters an error.
	///
	/// [`set_key`]: Self::set_key
	pub async fn delete_entry<B: Backend>(self, chart: &Starchart<B>) -> Result<bool, ActionError> {
		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let table = self.validate_table()?;
		let key = self.validate_key()?;

		self.check_table(backend).await?;
		self.check_metadata(backend).await?;

		if !backend
			.has(table, key)
			.await
			.map_err(ActionError::from_backend)?
		{
			drop(lock);
			return Ok(false);
		}

		backend
			.delete(table, key)
			.await
			.map_err(ActionError::from_backend)?;

		drop(lock);
		Ok(true)
	}

	/// Creates a table within the [`Starchart`].
	///
	/// # Errors
	///
	/// This raises an error if the [`Backend`] encounters an error.
	pub async fn create_table<B: Backend>(self, chart: &Starchart<B>) -> Result<(), ActionError> {
		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let table = self.validate_table()?;

		backend
			.ensure_table(table)
			.await
			.map_err(ActionError::from_backend)?;

		#[cfg(feature = "metadata")]
		{
			let metadata = D::default();
			backend
				.ensure(table, crate::METADATA_KEY, &metadata)
				.await
				.map_err(|e| ActionError {
					source: Some(Box::new(e)),
					kind: ActionErrorType::Metadata(Some(table.to_owned())),
				})?;
		}

		drop(lock);

		Ok(())
	}

	/// Reads a table from the [`Starchart`], returning a map-based iterator.
	///
	/// # Errors
	///
	/// This raises an error if the [`Backend`] encounters an error.
	pub async fn read_table<I: FromIterator<(String, D)>, B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<I, ActionError> {
		let lock = chart.guard.shared().await;

		let backend = &**chart;

		let table = self.validate_table()?;

		self.check_table(backend).await?;
		self.check_metadata(backend).await?;

		let data = backend
			.get_all(table)
			.await
			.map_err(ActionError::from_backend)?;

		drop(lock);

		Ok(data)
	}

	/// Deletes a table from the [`Starchart`], returning whether or not the table was actually deleted.
	///
	/// # Errors
	///
	/// This raises an error if the [`Backend`] encounters an error.
	pub async fn delete_table<B: Backend>(self, chart: &Starchart<B>) -> Result<bool, ActionError> {
		let lock = chart.guard.exclusive().await;

		let backend = &**chart;

		let table = self.validate_table()?;

		if self.check_table(backend).await.is_err() {
			drop(lock);
			return Ok(false);
		}

		self.check_metadata(backend).await?;

		backend
			.delete_table(table)
			.await
			.map_err(ActionError::from_backend)?;

		drop(lock);

		Ok(true)
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

	fn validate_key(&self) -> Result<&str, ActionError> {
		self.validate_metadata(self.key.as_deref())?;

		self.key.as_deref().ok_or(ActionError {
			source: None,
			kind: ActionErrorType::SomethingMissing(MissingValue::Key),
		})
	}

	fn validate_data(&self) -> Result<&'v D, ActionError> {
		self.data.ok_or(ActionError {
			source: None,
			kind: ActionErrorType::SomethingMissing(MissingValue::Data),
		})
	}

	fn validate_table(&self) -> Result<&str, ActionError> {
		self.validate_metadata(Some(self.table))?;

		Ok(self.table)
	}

	fn validate_entry(&self) -> Result<(&str, &'v D), ActionError> {
		Ok((self.validate_key()?, self.validate_data()?))
	}
}

impl<'v, D: IndexEntry + ?Sized> Action<'v, D> {
	/// Create an [`Action`] with the provided [`IndexEntry`].
	pub fn with_entry(table: &'v str, entry: &'v D) -> Self {
		let mut act = Self::new(table);

		act.set_entry(entry);

		act
	}

	/// Sets an [`IndexEntry`], which provides it's own key.
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

impl<'v, D: ?Sized> Default for Action<'v, D> {
	fn default() -> Self {
		Self {
			table: Default::default(),
			key: None,
			data: None,
		}
	}
}
