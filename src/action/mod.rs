#![allow(missing_copy_implementations)]

//! The action structs for CRUD operations.

mod kind;
pub mod result;
mod target;

use std::cell::Cell;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[doc(inline)]
pub use self::{kind::ActionKind, result::ActionResult, target::OperationTarget};
use crate::Entity;

/// An error occurred during validation of an [`Action`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ActionError {
	/// The [`OperationTarget`] was not set.
	#[error("an invalid operation was set")]
	InvalidOperation,
	/// No data was passed when data was expected.
	#[error("no data was given when data was expected")]
	NoData,
	/// No key was passed when a key was expected.
	#[error("no key was given when a key was expected.")]
	NoKey,
	/// Attempted to [`ActionKind::Update`] an [`OperationTarget::Table`].
	#[error("updating an entire table is unsupported")]
	UpdatingTable,
}

/// An [`Action`] for easy [`CRUD`] operations within a [`Gateway`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Gateway`]: crate::Gateway
#[must_use = "an action alone has no side effects"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action<S> {
	pub(crate) inner: InternalAction<S>,
	pub(crate) validated: Cell<bool>,
}

impl<S> Action<S> {
	/// Creates a new [`Action`] with the specified operation.
	pub const fn new(kind: ActionKind) -> Self {
		Self {
			inner: InternalAction::new(kind),
			validated: Cell::new(false),
		}
	}

	/// Returns the [`ActionKind`] we will be performing with said action.
	pub const fn kind(&self) -> ActionKind {
		self.inner.kind()
	}

	/// Returns the [`OperationTarget`] we will be performing with said action.
	#[must_use]
	pub const fn target(&self) -> OperationTarget {
		self.inner.target()
	}

	/// Whether the [`Action`] has been validated.
	#[must_use]
	pub fn is_validated(&self) -> bool {
		self.validated.get()
	}
}

// These are two separate blocks because we don't want
// any operations being created if `S` doesn't implement `Entity`.
impl<S: Entity> Action<S> {
	/// Begins a Create-based action.
	pub fn create() -> Self {
		Self::new(ActionKind::Create)
	}

	/// Begins a Read-based action.
	pub fn read() -> Self {
		Self::new(ActionKind::Read)
	}

	/// Begins an Update-based action.
	pub fn update() -> Self {
		Self::new(ActionKind::Update)
	}

	/// Begins a Delete-based action.
	pub fn delete() -> Self {
		Self::new(ActionKind::Delete)
	}

	/// Sets an [`OperationTarget`] for the action.
	///
	/// # Panics
	///
	/// Panics if the [`OperationTarget`] is an [`OperationTarget::Unknown`].
	pub fn set_target(&mut self, target: OperationTarget) -> &mut Self {
		assert!(
			target != OperationTarget::Unknown,
			"an unknown operation target was set"
		);
		self.inner.set_target(target);

		self.validated.set(false);

		self
	}

	/// Validates the [`Action`].
	///
	/// This is a no-op if the [`Action`] has already been validated.
	///
	/// # Errors
	///
	/// Returns an [`ActionError::InvalidOperation`] if the [`Action`] has not set an [`OperationTarget`].
	pub fn validate(&self) -> Result<(), ActionError> {
		if self.is_validated() {
			return Ok(());
		}

		if self.target() == OperationTarget::Unknown {
			return Err(ActionError::InvalidOperation);
		}

		if self.needs_data() && self.inner.data.is_none() {
			return Err(ActionError::NoData);
		}

		if self.needs_key() && self.inner.key.is_none() {
			return Err(ActionError::NoKey);
		}

		if self.is_updating_table() {
			return Err(ActionError::UpdatingTable);
		}

		self.validated.set(true);

		Ok(())
	}

	/// Sets the key for the action.
	///
	/// Users should prefer to call [`Self::set_data`] over this, as setting the
	/// data will automatically call this.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_key(&mut self, key: &S) -> &mut Self {
		self.inner.set_key(key.to_key());

		self.validated.set(false);

		self
	}

	/// Sets the data for the action.
	///
	/// This is unused on [`OperationTarget::Table`] actions.
	pub fn set_data(&mut self, entity: &S) -> &mut Self {
		self.set_key(entity);
		self.inner.set_entity(Box::new(entity.clone()));

		self
	}

	// Updating tables is unsupported
	fn is_updating_table(&self) -> bool {
		self.kind() == ActionKind::Update && self.target() == OperationTarget::Table
	}

	fn needs_data(&self) -> bool {
		if self.kind() == ActionKind::Read {
			return false;
		}

		if self.kind() == ActionKind::Delete {
			return false;
		}

		if self.target() == OperationTarget::Table {
			return false;
		}

		true
	}

	fn needs_key(&self) -> bool {
		if self.target() == OperationTarget::Table {
			return false;
		}

		true
	}
}

impl<S: Entity> Default for Action<S> {
	fn default() -> Self {
		Self {
			inner: InternalAction::default(),
			validated: Cell::default(),
		}
	}
}

// This struct is used for database creation and interaction
// within the crate, and performs no validation
// to ensure optimizations, and SHOULD NOT be exposed to public API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InternalAction<S> {
	kind: ActionKind,
	table_name: Option<String>,
	data: Option<Box<S>>,
	key: Option<String>,
	target: OperationTarget,
}

impl<S> InternalAction<S> {
	pub(crate) const fn new(kind: ActionKind) -> Self {
		Self {
			kind,
			table_name: None,
			data: None,
			key: None,
			target: OperationTarget::Unknown,
		}
	}

	pub(crate) const fn kind(&self) -> ActionKind {
		self.kind
	}

	pub(crate) const fn target(&self) -> OperationTarget {
		self.target
	}
}

impl<S: Entity> InternalAction<S> {
	pub(crate) fn set_table_name(&mut self, table_name: String) -> &mut Self {
		self.table_name = Some(table_name);

		self
	}

	pub(crate) fn set_key(&mut self, key: String) -> &mut Self {
		self.key = Some(key);

		self
	}

	pub(crate) fn set_entity(&mut self, entity: Box<S>) -> &mut Self {
		self.data = Some(entity);

		self
	}

	pub(crate) fn set_data(&mut self, data: S) -> &mut Self {
		self.data = Some(Box::new(data));

		self
	}

	pub(crate) fn set_target(&mut self, target: OperationTarget) -> &mut Self {
		self.target = target;

		self
	}
}

impl<S: Entity> Default for InternalAction<S> {
	fn default() -> Self {
		Self {
			kind: ActionKind::default(),
			table_name: Option::default(),
			data: Option::default(),
			key: Option::default(),
			target: OperationTarget::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use serde::{de::DeserializeOwned, Deserialize, Serialize};
	use static_assertions::assert_impl_all;

	use super::{Action, ActionKind, OperationTarget};
	use crate::Entity;

	#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
	struct Settings {
		key: u32,
		value: bool,
		test: u8,
	}

	impl Entity for Settings {
		fn to_key(&self) -> String {
			self.key.to_string()
		}
	}

	assert_impl_all!(
		Settings: Debug,
		Clone,
		Copy,
		PartialEq,
		Serialize,
		DeserializeOwned,
		Entity
	);

	#[test]
	fn basic() {
		let action = Action::<Settings>::new(ActionKind::Create);

		assert_eq!(action.kind(), ActionKind::Create);
		assert_eq!(action.target(), OperationTarget::Unknown);

		assert!(action.inner.table_name.is_none());
		assert!(action.inner.data.is_none());

		assert!(action.validate().is_err());
	}

	#[test]
	fn crud_constructors() {
		let create = Action::<Settings>::create();

		assert_eq!(create.kind(), ActionKind::Create);

		let read = Action::<Settings>::read();

		assert_eq!(read.kind(), ActionKind::Read);

		let update = Action::<Settings>::update();

		assert_eq!(update.kind(), ActionKind::Update);

		let delete = Action::<Settings>::delete();

		assert_eq!(delete.kind(), ActionKind::Delete);
	}

	#[test]
	fn default() {
		let action = Action::<Settings>::default();

		assert_eq!(action.kind(), ActionKind::Read);

		assert!(action.inner.table_name.is_none());

		assert!(action.inner.data.is_none());

		assert!(action.validate().is_err());

		assert_eq!(action.target(), OperationTarget::Unknown);
	}

	#[test]
	fn validate() {
		let mut action = Action::<Settings>::new(ActionKind::Read);

		assert!(action.validate().is_err());

		assert!(!action.is_validated());

		action.set_target(OperationTarget::Table);

		assert!(action.validate().is_ok());

		assert!(action.is_validated());

		action.validate().unwrap();

		assert!(action.is_validated());

		let new_action = action.set_target(OperationTarget::Entity);

		assert!(!new_action.is_validated());

		new_action.set_data(&Settings {
			key: 7,
			value: false,
			test: 74,
		});

		assert!(new_action.validate().is_ok());
	}
}
