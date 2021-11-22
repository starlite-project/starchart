#![feature(never_type, doc_cfg, try_trait_v2)]
#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::undocumented_unsafe_blocks,
	missing_copy_implementations,
	missing_docs
)]
#![deny(clippy::all)]
#![allow(
	clippy::module_name_repetitions,
	clippy::no_effect_underscore_binding,
	deprecated
)]
#![cfg_attr(not(test), warn(clippy::panic_in_result_fn))]
//! A simple database system that allows the use of multiple different backends.

pub mod action;
pub mod backend;
mod database;
mod entry;
pub mod error;
pub mod gateway;

#[doc(inline)]
pub use self::{action::Action, error::ChartError as Error, gateway::Gateway};
pub use self::{
	database::Database,
	entry::{Entry, IndexEntry, Key},
};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type ChartResult<T, B> = Result<T, Error<B>>;

/// The helper derive macro for easily implementing [`IndexEntry`].
#[cfg(feature = "derive")]
#[doc(cfg(feature = "derive"))]
pub use starchart_derive::IndexEntry;
