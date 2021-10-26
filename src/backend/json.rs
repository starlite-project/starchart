use super::Backend;
use async_trait::async_trait;
use futures_util::StreamExt;
use std::{
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

/// todo
#[doc(cfg(feature = "json"))]
#[derive(Debug, Error)]
pub enum JsonError {
    /// todo
    #[error("path {0} is not a directory")]
    PathNotDirectory(PathBuf),
    /// todo
    #[error("an IO error occurred: {0}")]
    Io(#[from] io::Error),
}

/// todo
#[doc(cfg(feature = "json"))]
#[derive(Debug, Default, Clone)]
pub struct JsonBackend {
    base_directory: PathBuf,
}

impl JsonBackend {
    /// todo
    ///
    /// # Errors
    ///
    /// todo
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
        let path = path.as_ref().to_path_buf();

        if path.is_dir() {
            Ok(Self {
                base_directory: path,
            })
        } else {
            Err(JsonError::PathNotDirectory(path))
        }
    }

    fn resolve_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.base_directory.clone().join(path)
    }
}

#[async_trait]
impl Backend for JsonBackend {
    type Error = JsonError;

    async fn init(&self) -> Result<(), JsonError> {
        if fs::read_dir(&self.base_directory).await.is_err() {
            fs::create_dir_all(&self.base_directory).await?;
        }

        Ok(())
    }

    async fn has_table(&self, table: &str) -> Result<bool, Self::Error> {
        let result = fs::read_dir(self.resolve_path(table)).await;

        match result {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    async fn create_table(&self, table: &str) -> Result<(), Self::Error> {
        fs::create_dir(self.resolve_path(table)).await?;

        Ok(())
    }

    async fn delete_table(&self, table: &str) -> Result<(), Self::Error> {
        if self.has_table(table).await? {
            fs::remove_dir_all(self.resolve_path(table)).await?;
        }

        Ok(())
    }

    async fn get_keys(&self, table: &str) -> Result<Vec<String>, Self::Error> {
        let mut stream = ReadDirStream::new(fs::read_dir(self.resolve_path(table)).await?);
        let mut output = Vec::new();

        while let Some(raw) = stream.next().await {
            let entry = raw?;

            let val = (*entry.file_name().to_string_lossy()).to_string();
            output.push(val);
        }

        Ok(output)
    }
}
