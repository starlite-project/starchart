//! Futures for [`Backend`] functions to return, for easier documentation.
//!
//! [`Backend`]: crate::backend::Backend
use std::{future::Future, pin::Pin};

#[cfg(doc)]
use crate::backend::Backend;

/// The future returned from [`Backend::init`].
pub type InitFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::shutdown`].
pub type ShutdownFuture<'a> = PinBoxFuture<'a>;

/// The future returned from [`Backend::has_table`].
pub type HasTableFuture<'a, E> = PinBoxFuture<'a, Result<bool, E>>;

/// The future returned from [`Backend::create_table`].
pub type CreateTableFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::delete_table`].
pub type DeleteTableFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::ensure_table`].
pub type EnsureTableFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::get_all`].
pub type GetAllFuture<'a, I, E> = PinBoxFuture<'a, Result<I, E>>;

/// The future returned from [`Backend::get_keys`].
pub type GetKeysFuture<'a, I, E> = PinBoxFuture<'a, Result<I, E>>;

/// The future returned from [`Backend::get`].
pub type GetFuture<'a, D, E> = PinBoxFuture<'a, Result<Option<D>, E>>;

/// The future returned from [`Backend::has`].
pub type HasFuture<'a, E> = PinBoxFuture<'a, Result<bool, E>>;

/// The future returned from [`Backend::create`].
pub type CreateFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::ensure`].
pub type EnsureFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::update`].
pub type UpdateFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::replace`].
pub type ReplaceFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// The future returned from [`Backend::delete`].
pub type DeleteFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

type PinBoxFuture<'a, Rt = ()> = Pin<Box<dyn Future<Output = Rt> + Send + 'a>>;
