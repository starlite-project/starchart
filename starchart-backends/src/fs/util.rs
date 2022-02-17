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
	use std::{ffi::OsStr, fs::remove_dir_all, io::ErrorKind, path::{Path, PathBuf}};

	use serde::{Deserialize, Serialize};
	use starchart::IndexEntry;
	use tokio::sync::RwLock;

	pub static TEST_GUARD: RwLock<()> = RwLock::const_new(());

	#[derive(Debug)]
	#[repr(transparent)]
	pub struct TestPath(PathBuf);

	impl TestPath {
		pub fn new(test_name: &str, module_name: &str) -> Self {
			let mut path = PathBuf::from(env!("OUT_DIR"));
			path.extend(&["tests", module_name, test_name]);

			let res = remove_dir_all(&path);

			if let Err(e) = res {
				if e.kind() == ErrorKind::NotFound {
					// noop
				} else {
					panic!("{:?}", e);
				}
			}

			Self(path)
		}
	}

	impl AsRef<Path> for TestPath {
		fn as_ref(&self) -> &Path {
			self.0.as_ref()
		}
	}

	impl AsRef<OsStr> for TestPath {
		fn as_ref(&self) -> &OsStr {
			self.0.as_ref()
		}
	}

	#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
	pub struct MockSettings {
		pub id: u32,
		pub value: String,
		pub array: Vec<u8>,
		pub opt: Option<f64>,
	}

	impl MockSettings {
		pub fn new() -> Self {
			Self {
				id: 1,
				value: "hello, world!".to_owned(),
				array: vec![1, 2, 3, 4, 5],
				opt: Some(4.2),
			}
		}
	}

	impl IndexEntry for MockSettings {
		type Key = u32;

		fn key(&self) -> &Self::Key {
			&self.id
		}
	}
}
