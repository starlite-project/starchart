use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("path {0} is not a directory")]
    PathNotDirectory(PathBuf),
}

#[cfg(feature = "json")]
#[doc(cfg(feature = "json"))]
#[derive(Debug, Default, Clone)]
pub struct JsonBackend {
    base_directory: PathBuf,
}

impl JsonBackend {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
        let path = path.as_ref().to_path_buf();

        if path.is_dir() {
            Ok(Self { base_directory: path })
        } else {
            Err(JsonError::PathNotDirectory(path))
        }
    }
}
