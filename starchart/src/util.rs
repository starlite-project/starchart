#![allow(clippy::missing_safety_doc)]

#[cfg(not(has_unwrap_unchecked))]
use std::hint::unreachable_unchecked;

#[cfg(feature = "metadata")]
pub fn is_metadata(key: &str) -> bool {
	key == crate::METADATA_KEY
}

#[cfg(not(feature = "metadata"))]
pub fn is_metadata(_: &str) -> bool {
	false
}

pub unsafe trait InnerUnwrap<T> {
	unsafe fn inner_unwrap(self) -> T;
}

#[cfg(not(has_unwrap_unchecked))]
unsafe impl<T> InnerUnwrap<T> for Option<T> {
	#[inline]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		debug_assert!(self.is_some());
		self.map_or_else(|| unreachable_unchecked(), |v| v)
	}
}

#[cfg(has_unwrap_unchecked)]
unsafe impl<T> InnerUnwrap<T> for Option<T> {
	#[allow(clippy::inline_always)]
	#[inline(always)]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		self.unwrap_unchecked()
	}
}

#[cfg(not(has_unwrap_unchecked))]
unsafe impl<T, E> InnerUnwrap<T> for Result<T, E> {
	#[inline]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		debug_assert!(self.is_ok());
		if let Ok(v) = self {
			v
		} else {
			unreachable_unchecked()
		}
	}
}

#[cfg(has_unwrap_unchecked)]
unsafe impl<T, E> InnerUnwrap<T> for Result<T, E> {
	#[allow(clippy::inline_always)]
	#[inline(always)]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		self.unwrap_unchecked()
	}
}
