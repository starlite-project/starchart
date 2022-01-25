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
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(not(test), warn(clippy::panic_in_result_fn))]
//! A simple database system that allows the use of multiple different backends.

use std::result::Result as StdResult;

#[cfg(not(any(feature = "accessor", feature = "action")))]
compile_error!("either the \"action\" feature or the \"accessor\" feature must be enabled.");

#[cfg(all(feature = "metadata", not(tarpaulin_include)))]
pub(crate) const METADATA_KEY: &str = "__metadata__";

#[cfg(feature = "accessor")]
pub mod accessor;
#[cfg(feature = "action")]
pub mod action;
mod atomics;
pub mod backend;
mod entry;
pub mod error;
mod starchart;
#[cfg(not(tarpaulin_include))]
mod util;

#[cfg(feature = "accessor")]
#[doc(inline)]
pub use self::accessor::Accessor;
#[cfg(feature = "action")]
#[doc(inline)]
pub use self::action::Action;
#[doc(inline)]
pub use self::{
	entry::{Entry, IndexEntry, Key},
	error::Error,
	starchart::Starchart,
};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type Result<T, E = Error> = StdResult<T, E>;

/// The helper derive macro for easily implementing [`IndexEntry`].
#[cfg(feature = "derive")]
pub use starchart_derive::IndexEntry;
