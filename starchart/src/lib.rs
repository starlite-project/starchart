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
#![allow(clippy::module_name_repetitions, clippy::no_effect_underscore_binding)]
#![cfg_attr(
	docsrs,
	feature(doc_auto_cfg, doc_cfg),
	deny(rustdoc::broken_intra_doc_links)
)]
#![cfg_attr(not(test), warn(clippy::panic_in_result_fn))]
//! A simple database system that allows the use of multiple different backends.

#[cfg(feature = "metadata")]
const METADATA_KEY: &str = "__metadata__";

use std::result::Result as StdResult;

pub mod action;
mod atomics;
pub mod backend;
mod entry;
pub mod error;
mod starchart;
#[cfg(not(tarpaulin_include))]
mod util;

#[doc(inline)]
pub use self::{
	action::Action,
	backend::Backend,
	entry::{Entry, IndexEntry, Key},
	error::Error,
	starchart::Starchart,
};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type Result<T, E = Error> = StdResult<T, E>;

/// The helper derive macro for easily implementing [`IndexEntry`].
#[cfg(feature = "derive")]
pub use starchart_derive::IndexEntry;
