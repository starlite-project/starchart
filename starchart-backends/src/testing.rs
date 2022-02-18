#[cfg(all(feature = "fs", not(miri)))]
use std::{
	ffi::OsStr,
	fs::remove_dir_all,
	io::ErrorKind,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
#[cfg(all(feature = "fs", not(miri)))]
use tokio::sync::Mutex;

#[cfg(all(feature = "fs", not(miri)))]
pub static TEST_GUARD: Mutex<()> = Mutex::const_new(());

#[derive(Debug)]
#[repr(transparent)]
#[cfg(all(feature = "fs", not(miri)))]
pub struct TestPath(PathBuf);

#[cfg(all(feature = "fs", not(miri)))]
impl TestPath {
	pub fn new(test_name: &str, module_name: &str) -> Self {
		let mut path = PathBuf::from(env!("OUT_DIR"));
		path.extend(&[test_name, module_name]);

		if let Err(e) = remove_dir_all(&path) {
			if e.kind() == ErrorKind::NotFound {
				// noop
			} else {
				panic!("{:?}", e);
			}
		}

		Self(path)
	}
}

#[cfg(all(feature = "fs", not(miri)))]
impl AsRef<Path> for TestPath {
	fn as_ref(&self) -> &Path {
		self.0.as_ref()
	}
}

#[cfg(all(feature = "fs", not(miri)))]
impl AsRef<OsStr> for TestPath {
	fn as_ref(&self) -> &OsStr {
		self.0.as_ref()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSettings {
	pub id: u32,
	pub value: String,
	pub array: Vec<u8>,
	pub opt: Option<f64>,
}

impl Default for TestSettings {
	fn default() -> Self {
		Self {
			id: 1,
			value: "hello, world!".to_owned(),
			array: vec![1, 2, 3, 4, 5],
			opt: Some(4.2),
		}
	}
}
