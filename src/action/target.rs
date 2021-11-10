use serde::{Deserialize, Serialize};

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