#![allow(clippy::missing_safety_doc)]

#[cfg(no_unwrap_unchecked)]
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

#[cfg(no_unwrap_unchecked)]
unsafe impl<T> InnerUnwrap<T> for Option<T> {
	#[inline]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		debug_assert!(self.is_some());
		self.map_or_else(|| unreachable_unchecked(), |v| v)
	}
}

#[cfg(not(no_unwrap_unchecked))]
unsafe impl<T> InnerUnwrap<T> for Option<T> {
	#[allow(clippy::inline_always)]
	#[inline(always)]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		self.unwrap_unchecked()
	}
}

#[cfg(no_unwrap_unchecked)]
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

#[cfg(not(no_unwrap_unchecked))]
unsafe impl<T, E> InnerUnwrap<T> for Result<T, E> {
	#[allow(clippy::inline_always)]
	#[inline(always)]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		self.unwrap_unchecked()
	}
}

#[cfg(all(test, feature = "fs"))]
pub mod testing {
	use std::{
		ffi::OsStr,
		fs,
		io::Result as IoResult,
		path::{Path, PathBuf},
	};

	use crate::atomics::Guard;

	pub static TEST_GUARD: Guard = Guard::new();

	#[derive(Debug, Clone)]
	#[repr(transparent)]
	pub struct FsCleanup(PathBuf);

	impl FsCleanup {
		pub fn new(test_name: &str, module: &str, should_create: bool) -> IoResult<Self> {
			let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.join("target")
				.join("tests")
				.join(module)
				.join(test_name);

			if should_create {
				fs::create_dir_all(&path)?;
			}

			Ok(Self(path))
		}
	}

	impl AsRef<Path> for FsCleanup {
		fn as_ref(&self) -> &Path {
			self.0.as_ref()
		}
	}

	impl AsRef<OsStr> for FsCleanup {
		fn as_ref(&self) -> &OsStr {
			self.0.as_ref()
		}
	}

	impl Drop for FsCleanup {
		#[allow(clippy::let_underscore_drop)]
		fn drop(&mut self) {
			let _ = fs::remove_dir_all(&self.0);
		}
	}
}
