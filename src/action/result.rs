//! Represents the many different results from actions.

#![allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]

use std::ops::Deref;

use thiserror::Error;

use crate::{Entity, NeverEntity};

use super::OperationTarget;

/// The base [`Result`] type for [`Action`]s.
///
/// [`Action`]: crate::action::Action
#[derive(Debug)]
#[must_use = "this `ActionResult` may be an Error of some kind, which should be handled"]
pub enum ActionResult<T: Entity = NeverEntity> {
    /// The result from an [`Action::create`].
    ///
    /// [`Action::create`]: crate::action::Action::create
    Create(CreateResult),
    /// The result from an [`Action::read`].
    ///
    /// [`Action::read`]: crate::action::Action::read
    Read(ReadResult<T>),
    /// The result from an [`Action::update`].
    ///
    /// [`Action::update`]: crate::action::Action::update
    Update(UpdateResult),
    /// The result from an [`Action::delete`].
    ///
    /// [`Action::delete`]: crate::action::Action::delete
    Delete(DeleteResult),
}

impl ActionResult<NeverEntity> {
    /// Converts from [`ActionResult`] to [`Option`].
    ///
    /// This consumes `self`, returning a [`CreateResult`] if the [`ActionResult`] is a [`CreateResult`],
    /// and [`None`] otherwise.
    pub fn create(self) -> Option<CreateResult> {
        match self {
            ActionResult::Create(result) => Some(result),
            _ => None,
        }
    }

    /// Converts from [`ActionResult`] to [`Option`].
    ///
    /// This consumes `self`, returning an [`UpdateResult`] if the [`ActionResult`] is an [`UpdateResult`],
    /// and [`None`] otherwise.
    pub fn update(self) -> Option<UpdateResult> {
        match self {
            ActionResult::Update(result) => Some(result),
            _ => None,
        }
    }
}

impl<T: Entity> ActionResult<T> {
    /// Converts from [`ActionResult`] to [`Option`].
    ///
    /// This consumes `self`, returning a [`ReadResult`] if the [`ActionResult`] is a [`ReadResult`],
    /// and [`None`] otherwise.
    pub fn read(self) -> Option<ReadResult<T>> {
        match self {
            ActionResult::Read(result) => Some(result),
            _ => None,
        }
    }
}

/// A result from an [`Action::create`].
///
/// [`Action::create`]: crate::action::Action::create
#[derive(Debug)]
#[must_use = "this `CreateResult` may be an Error of some kind, which should be handled"]
pub enum CreateResult {
    /// A table creation result.
    Table(Result<(), CreateError>),
    /// An entity creation result.
    Entity(Result<(), CreateError>),
}

impl Deref for CreateResult {
    type Target = Result<(), CreateError>;

    fn deref(&self) -> &Self::Target {
        match self {
            CreateResult::Table(r) | CreateResult::Entity(r) => r,
        }
    }
}

/// An error occurred during an [`Action::create`].
///
/// [`Action::create`]: crate::action::Action::create
#[derive(Debug, Error)]
#[error("an error happened during {target} creation")]
pub struct CreateError {
    source: Box<dyn std::error::Error>,
    target: OperationTarget,
}

/// A result from an [`Action::read`].
///
/// [`Action::read`]: crate::action::Action::read
#[derive(Debug)]
#[must_use = "this `ReadResult` may be an Error of some kind, which should be handled"]
pub enum ReadResult<T: Entity> {
    /// A table read result.
    Table(Result<Vec<T>, ReadError>),
    /// An entity read result.
    Entity(Result<T, ReadError>),
}

/// An error occurred during an [`Action::read`].
///
/// [`Action::read`]: crate::action::Action::read
#[derive(Debug, Error)]
#[error("an error happened during {target} read")]
pub struct ReadError {
    source: Box<dyn std::error::Error>,
    target: OperationTarget,
}

/// A result from an [`Action::update`].
///
/// [`Action::update`]: crate::action::Action::update
#[derive(Debug)]
#[must_use = "this `UpdateResult` may be an Error of some kind, which should be handled"]
pub enum UpdateResult {
    /// A table update result.
    Table(Result<(), UpdateError>),
    /// An entity update result.
    Entity(Result<(), UpdateError>),
}

impl Deref for UpdateResult {
    type Target = Result<(), UpdateError>;

    fn deref(&self) -> &Self::Target {
        match self {
            UpdateResult::Table(r) | UpdateResult::Entity(r) => r,
        }
    }
}

/// An error occurred during an [`Action::update`].
///
/// [`Action::update`]: crate::action::Action::update
#[derive(Debug, Error)]
#[error("an error happened during {target} update")]
pub struct UpdateError {
    source: Box<dyn std::error::Error>,
    target: OperationTarget,
}

/// A result from an [`Action::delete`].
///
/// [`Action::delete`]: crate::action::Action::delete
#[derive(Debug)]
#[must_use = "this `DeleteResult` may be an Error of some kind, which should be handled"]
pub enum DeleteResult {
    /// A table delete result.
    Table(Result<(), DeleteError>),
    /// An entity delete result.
    Entity(Result<(), DeleteError>),
}

impl Deref for DeleteResult {
    type Target = Result<(), DeleteError>;

    fn deref(&self) -> &Self::Target {
        match self {
            DeleteResult::Table(r) | DeleteResult::Entity(r) => r,
        }
    }
}

/// An error occurred during an [`Action::delete`].
///
/// [`Action::delete`]: crate::action::Action::delete
#[derive(Debug, Error)]
#[error("an error happened during {target} deletion")]
pub struct DeleteError {
    source: Box<dyn std::error::Error>,
    target: OperationTarget,
}
