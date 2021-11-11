#![feature(never_type, doc_cfg)]
#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	clippy::str_to_string,
	clippy::string_to_string,
	missing_copy_implementations,
	missing_docs
)]
#![deny(clippy::all)]
#![allow(
	clippy::module_name_repetitions,
	clippy::no_effect_underscore_binding,
	dead_code,
	deprecated
)]
#![cfg_attr(not(test), warn(clippy::panic_in_result_fn))]
//! A simple database system that allows the use of multiple different backends.

pub mod action;
#[cfg(not(feature = "unstable-atomics"))]
pub(crate) mod atomics;
#[cfg(feature = "unstable-atomics")]
#[doc(cfg(feature = "unstable-atomics"))]
pub mod atomics;
pub mod backend;
mod database;
mod entry;
pub mod error;
pub mod gateway;
#[cfg(test)]
mod test_utils;

#[doc(inline)]
pub use self::{action::Action, error::ChartError as Error, gateway::Gateway};
pub use self::{database::Database, entry::Entity};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type ChartResult<T, B> = Result<T, Error<B>>;

/// The helper derive macro for easily implementing [`Entity`].
#[cfg(feature = "derive")]
#[doc(cfg(feature = "derive"))]
pub use starchart_derive::Entity;
