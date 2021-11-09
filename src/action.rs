#![allow(missing_copy_implementations)]

//! The action structs for CRUD operations.

use crate::Entity;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
#[doc(hidden)]
pub enum ActionError {
    #[error("an invalid operation was set")]
    InvalidOperation,
}

/// An [`Action`] for easy [`CRUD`] operations within a [`Gateway`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Gateway`]: crate::Gateway
#[must_use = "an action alone has no side effects"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action<S> {
    kind: ActionKind,
    table_name: Option<S>,
    data: Option<Box<S>>,
    target: OperationTarget,
    validated: bool,
}

impl<S> Action<S> {
    /// Creates a new [`Action`] with the specified operation.
    pub const fn new(kind: ActionKind) -> Self {
        Self {
            kind,
            table_name: None,
            data: None,
            target: OperationTarget::Unknown,
            validated: false,
        }
    }

    /// Returns the [`ActionKind`] we will be performing with said action.
    pub const fn kind(&self) -> ActionKind {
        self.kind
    }

    /// Returns the [`OperationTarget`] we will be performing with said action.
    pub const fn target(&self) -> OperationTarget {
        self.target
    }

    /// Whether the [`Action`] has been validated.
    pub const fn is_validated(&self) -> bool {
        self.validated
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
            !(target == OperationTarget::Unknown),
            "an unknown operation target was set"
        );
        self.target = target;

        self.validated = false;

        self
    }

    /// Validates the [`Action`].
    ///
    /// This is a no-op if the [`Action`] has already been validated.
    ///
    /// # Errors
    ///
    /// Returns an [`ActionError::InvalidOperation`] if the [`Action`] has not set an [`OperationTarget`].
    pub fn validate(&mut self) -> Result<(), ActionError> {
        if self.validated {
            return Ok(());
        }

        if self.target == OperationTarget::Unknown {
            return Err(ActionError::InvalidOperation);
        }

        self.validated = true;

        Ok(())
    }
}

impl<S: Entity> Default for Action<S> {
    fn default() -> Self {
        Self {
            kind: ActionKind::default(),
            table_name: Option::default(),
            data: Option::default(),
            target: OperationTarget::default(),
            validated: bool::default(),
        }
    }
}

/// The type of [`CRUD`] action to perform
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
#[must_use = "getting the information on what action will be performed has no side effects"]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ActionKind {
    /// Signifies that the operation will be a Create.
    ///
    /// This locks the database and allows no other reads or writes until it is complete.
    Create,
    /// Signifies that the operation will be a Read.
    ///
    /// This allows multiple different readers, but doesn't allow writing until all Reads are complete.
    Read,
    /// Signifies that the operation will be an Update.
    ///
    /// This locks the database and allows no other reads or writes until it is complete.
    Update,
    /// Signifies that the operation will be a Delete.
    ///
    /// This locks the database and allows no other reads or writes until it is complete.
    Delete,
}

impl Default for ActionKind {
    fn default() -> Self {
        Self::Read
    }
}

/// The target of the [`CRUD`] operation.
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OperationTarget {
    /// The operation will be performed on a table.
    Table,
    /// The operation will be performed on a single entity.
    Entity,
    /// An unknown operation will occur, this raises an error if it's set when [`Action::validate`] is called.
    Unknown,
}

impl Default for OperationTarget {
    fn default() -> Self {
        Self::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, ActionKind, OperationTarget};
    use crate::{Entity, Key};
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    struct Settings {
        key: u32,
        value: bool,
        test: u8,
    }

    impl Key for Settings {
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
        let mut action = Action::<Settings>::new(ActionKind::Create);

        assert_eq!(action.kind(), ActionKind::Create);
        assert_eq!(action.target(), OperationTarget::Unknown);

        assert!(action.table_name.is_none());
        assert!(action.data.is_none());

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
        let mut action = Action::<Settings>::default();

        assert_eq!(action.kind(), ActionKind::Read);

        assert!(action.table_name.is_none());

        assert!(action.data.is_none());

        assert!(action.validate().is_err());

        assert_eq!(action.target(), OperationTarget::Unknown);
    }

    #[test]
    fn validate() {
        let mut action = Action::<Settings>::new(ActionKind::Create);

        assert!(action.validate().is_err());

        assert!(!action.is_validated());

        action.set_target(OperationTarget::Table);

        assert!(action.validate().is_ok());

        assert!(action.is_validated());

        action.validate().unwrap();

        assert!(action.is_validated());

        let new_action = action.set_target(OperationTarget::Entity);

        assert!(!new_action.is_validated());

        assert!(new_action.validate().is_ok());
    }
}
