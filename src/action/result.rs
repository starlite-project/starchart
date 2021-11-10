//! Represents the many different results from actions.

use thiserror::Error;

/// The base [`Result`] type for [`Action`]s.
/// 
/// [`Action`]: crate::action::Action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionResult {
    /// The result from an [`Action::create`].
    /// 
    /// [`Action::create`]: crate::action::Action::create
    Create(CreateResult),
    /// The result from an [`Action::read`].
    /// 
    /// [`Action::read`]: crate::action::Action::read
    Read(ReadResult),
    /// The result from an [`Action::update`].
    /// 
    /// [`Action::update`]: crate::action::Action::update
    Update(UpdateResult),
    /// The result from an [`Action::delete`].
    /// 
    /// [`Action::delete`]: crate::action::Action::delete
    Delete(DeleteResult),
}

/// A result from an [`Action::create`].
/// 
/// [`Action::create`]: crate::action::Action::create
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateResult {
}

#[derive(Debug, Error)]
#[error("an error happened during {.type} creation")]
pub struct CreateError {
    source: Box<dyn std::error::Error>,
}

/// A result from an [`Action::read`].
/// 
/// [`Action::read`]: crate::action::Action::read
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadResult {}

/// A result from an [`Action::update`].
/// 
/// [`Action::update`]: crate::action::Action::update
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateResult {}

/// A result from an [`Action::delete`].
/// 
/// [`Action::delete`]: crate::action::Action::delete
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeleteResult {}