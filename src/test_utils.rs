use std::{
	ffi::OsStr,
	fs,
	io::Result as IoResult,
	path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct FsCleanup(PathBuf);

impl FsCleanup {
	pub fn new(test_name: &str, should_create: bool) -> IoResult<Self> {
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.join("target")
			.join("tests")
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
