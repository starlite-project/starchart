#![feature(option_result_unwrap_unchecked, never_type, doc_cfg)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::suspicious,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::panic_in_result_fn,
    missing_copy_implementations
)]
#![deny(clippy::all, missing_docs)]
#![allow(
    clippy::module_name_repetitions,
    clippy::no_effect_underscore_binding,
    dead_code
)]
//! A simple database system that allows the use of multiple different backends.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod backend;
mod database;
pub mod error;
pub mod gateway;

/// A marker trait for use within the [`Database`].
///
/// This signifies that the type can be stored within a [`Database`].
pub trait Settings {}

impl<T> Settings for T where T: Serialize + for<'de> Deserialize<'de> + Debug {}

pub use self::database::Database;
#[doc(inline)]
pub use self::{error::ChartError as Error, gateway::Gateway};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type ChartResult<T> = Result<T, Error>;
