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
#![cfg_attr(docsrs, feature(doc_auto_cfg), deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(not(test), warn(clippy::panic_in_result_fn))]
//! All the basic backends for the starchart crate

#[cfg(feature = "fs")]
pub mod fs;
