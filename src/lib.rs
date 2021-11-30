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
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), warn(clippy::panic_in_result_fn))]
//! A simple database system that allows the use of multiple different backends.

pub mod action;
pub mod backend;
mod entry;
pub mod error;
mod starchart;
#[cfg(test)]
pub(crate) mod test_utils;
#[cfg(not(tarpaulin_include))]
mod util;

#[doc(inline)]
pub use self::{
	action::Action,
	entry::{Entry, IndexEntry, Key},
	error::ChartError as Error,
	starchart::Starchart,
};

/// A type alias for a [`Result`] that wraps around [`Error`].
pub type ChartResult<T, B> = Result<T, Error<B>>;

/// A type alias for a [`Starchart`].
#[deprecated(since = "0.8.0", note = "Gateway has been renamed to `Starchart`")]
pub type Gateway<B> = Starchart<B>;

/// The helper derive macro for easily implementing [`IndexEntry`].
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use starchart_derive::IndexEntry;
