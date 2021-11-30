#![allow(clippy::missing_safety_doc)]

use std::hint::unreachable_unchecked;

pub unsafe trait InnerUnwrap<T> {
	unsafe fn inner_unwrap(self) -> T;
}

unsafe impl<T> InnerUnwrap<T> for Option<T> {
	#[inline]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		debug_assert!(self.is_some());
		self.map_or_else(|| unreachable_unchecked(), |v| v)
	}
}

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

#[cfg(test)]
pub mod testing {
	#[cfg(feature = "fs")]
	use std::{
		ffi::OsStr,
		fs,
		io::Result as IoResult,
		path::{Path, PathBuf},
	};

	#[cfg(feature = "fs")]
	#[derive(Debug, Clone)]
	#[repr(transparent)]
	pub struct FsCleanup(PathBuf);

	#[cfg(feature = "fs")]
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

	#[cfg(feature = "fs")]
	impl AsRef<Path> for FsCleanup {
		fn as_ref(&self) -> &Path {
			self.0.as_ref()
		}
	}

	#[cfg(feature = "fs")]
	impl AsRef<OsStr> for FsCleanup {
		fn as_ref(&self) -> &OsStr {
			self.0.as_ref()
		}
	}

	#[cfg(feature = "fs")]
	impl Drop for FsCleanup {
		#[allow(clippy::let_underscore_drop)]
		fn drop(&mut self) {
			let _ = fs::remove_dir_all(&self.0);
		}
	}
}