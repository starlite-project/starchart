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

pub mod backend;
mod database;
mod entry;
pub mod error;
pub mod gateway;

pub use self::{
    database::Database,
    entry::{Key, Settings},
};
#[doc(inline)]
pub use self::{error::ChartError as Error, gateway::Gateway};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type ChartResult<T, B> = Result<T, Error<B>>;
