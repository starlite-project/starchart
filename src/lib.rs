#![feature(never_type, doc_cfg)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::suspicious,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::panic_in_result_fn,
    missing_copy_implementations,
    missing_docs
)]
#![deny(clippy::all)]
#![allow(
    clippy::module_name_repetitions,
    clippy::no_effect_underscore_binding,
    dead_code
)]
//! A simple database system that allows the use of multiple different backends.

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub mod backend;
mod database;
pub mod error;
pub mod gateway;

/// A marker trait for use within the [`Database`].
///
/// This signifies that the type can be stored within a [`Database`].
pub trait Settings: Serialize + DeserializeOwned + Debug {}

impl<T> Settings for T where T: Serialize + DeserializeOwned + Debug {}

pub use self::database::Database;
#[doc(inline)]
pub use self::{error::ChartError as Error, gateway::Gateway};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type ChartResult<T> = Result<T, Error>;
