use std::{ffi::OsStr, path::Path};

use super::{FsError, FsErrorType};

pub fn resolve_key(extension: &str, file_name: &OsStr) -> Result<String, FsError> {
	let path_ref: &Path = file_name.as_ref();

	if path_ref.extension().map_or(false, |path| path == extension) {
		path_ref
			.file_stem()
			.ok_or(FsError {
				source: None,
				kind: FsErrorType::InvalidFile(path_ref.to_path_buf()),
			})
			.map(|raw| raw.to_string_lossy().into_owned())
	} else {
		Err(FsError {
			source: None,
			kind: FsErrorType::InvalidFile(path_ref.to_path_buf()),
		})
	}
}

#[cfg(test)]
pub mod testing {
	use std::{ffi::OsStr, fs, io::Result as IoResult, path::{Path, PathBuf}};

	use tokio::sync::RwLock;

	pub static TEST_GUARD: RwLock<()> = RwLock::const_new(());

	#[derive(Debug)]
	#[repr(transparent)]
	pub struct FsCleanup(PathBuf);

	impl FsCleanup {
		pub fn new(test_name: &str, module_name: &str, should_create: bool) -> IoResult<Self> {
			let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
			path.extend(&["target", "tests", module_name, test_name]);

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
		fn drop(&mut self) {
			let _res = fs::remove_dir_all(&self.0);
		}
	}
}
