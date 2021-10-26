#![feature(doc_cfg)]
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
#![allow(clippy::module_name_repetitions, clippy::no_effect_underscore_binding)]
//! todo

pub mod backend;
pub mod error;

pub use self::error::ChartError as Error;

use self::backend::Backend;

/// todo
pub type ChartResult<T> = Result<T, Error>;

/// todo
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Database<B: Backend> {
    backend: B,
}
