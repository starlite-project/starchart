#![feature(option_result_unwrap_unchecked, never_type)]
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
#![cfg_attr(docsrs, feature(doc_cfg))]
//! A simple database system that allows the use of multiple different backends.

pub mod backend;
mod database;
pub mod error;
pub mod gateway;

pub use self::database::Database;
#[doc(inline)]
pub use self::{error::ChartError as Error, gateway::Gateway};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type ChartResult<T> = Result<T, Error>;
