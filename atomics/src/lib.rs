#![feature(never_type, doc_cfg)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::suspicious,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::panic_in_result_fn,
    missing_copy_implementations
)]
#![deny(clippy::all)]
#![allow(
    clippy::module_name_repetitions,
    clippy::no_effect_underscore_binding,
    dead_code,
    warnings
)]
#![cfg_attr(not(any(feature = "std", test)), no_std)]

pub use core::sync::atomic::{fence, Ordering};

use core::{cell::UnsafeCell, fmt};

#[cfg(feature = "std")]
use std::panic::RefUnwindSafe;

#[repr(transparent)]
pub struct Atomic<T> {
    inner: UnsafeCell<T>
}

unsafe impl<T: Copy + Send> Sync for Atomic<T> {}