#![allow(missing_copy_implementations)]

//! The action structs for CRUD operations.

use serde::{Deserialize, Serialize};

/// An [`Action`] for easy [`CRUD`] operations within a [`Gateway`].
///
/// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
/// [`Gateway`]: crate::Gateway
#[must_use = "an action alone has no side effects"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    kind: ActionKind,
}

impl Action {
    /// Returns the [`ActionKind`] we will be performing with said action.
    pub const fn kind(&self) -> ActionKind {
        self.kind
    }

    /// Begins a Create-based action.
    pub const fn create() -> Self {
        Self::new(ActionKind::Create)
    }

    /// Begins a Read-based action.
    pub const fn read() -> Self {
        Self::new(ActionKind::Read)
    }

    /// Begins an Update-based action.
    pub const fn update() -> Self {
        Self::new(ActionKind::Update)
    }

    /// Begins a Delete-based action.
    pub const fn delete() -> Self {
        Self::new(ActionKind::Delete)
    }

    /// Creates a new [`Action`] with the specified operation.
    pub const fn new(kind: ActionKind) -> Self {
        Self { kind }
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
