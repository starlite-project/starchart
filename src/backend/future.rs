//! todo

use std::{future::Future, pin::Pin};

/// todo
pub type InitFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type ShutdownFuture<'a> = PinBoxFuture<'a>;

/// todo
pub type HasTableFuture<'a, E> = PinBoxFuture<'a, Result<bool, E>>;

/// todo
pub type CreateTableFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type DeleteTableFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type EnsureTableFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type GetAllFuture<'a, I, E> = PinBoxFuture<'a, Result<I, E>>;

/// todo
pub type GetKeysFuture<'a, I, E> = PinBoxFuture<'a, Result<I, E>>;

/// todo
pub type GetFuture<'a, D, E> = PinBoxFuture<'a, Result<Option<D>, E>>;

/// todo
pub type HasFuture<'a, E> = PinBoxFuture<'a, Result<bool, E>>;

/// todo
pub type CreateFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type EnsureFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type UpdateFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type ReplaceFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

/// todo
pub type DeleteFuture<'a, E> = PinBoxFuture<'a, Result<(), E>>;

type PinBoxFuture<'a, Rt = ()> = Pin<Box<dyn Future<Output = Rt> + Send + 'a>>;
