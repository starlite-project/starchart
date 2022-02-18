#[cfg(test)]
pub mod testing;

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
