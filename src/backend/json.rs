use super::Backend;
use async_trait::async_trait;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
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
    /// todo
    #[error("a JSON error occurred")]
    SerdeJson(#[from] serde_json::Error),
    /// todo
    #[error("file {} is invalid", .0.display())]
    InvalidFile(PathBuf),
    /// todo
    #[error("file {} already exists", .0.display())]
    FileAlreadyExists(PathBuf),
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

        if path.is_file() {
            Err(JsonError::PathNotDirectory(path))
        } else {
            Ok(Self {
                base_directory: path,
            })
        }
    }

    fn resolve_path<P: AsRef<Path>>(&self, path: &[P]) -> PathBuf {
        let mut base = self.base_directory.clone();

        for value in path {
            base = base.join(value);
        }

        base
    }

    fn resolve_key(file: OsString) -> Result<String, JsonError> {
        let path: PathBuf = file.into();

        let mut stringified = path.display().to_string();

        if stringified
            .rsplit('.')
            .next()
            .map(|ext| ext.eq_ignore_ascii_case("json"))
            == Some(true)
        {
            let range = unsafe { stringified.rfind(".json").unwrap_unchecked().. };

            stringified.replace_range(range, "");

            Ok(stringified)
        } else {
            Err(JsonError::InvalidFile(path))
        }
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
        let result = fs::read_dir(self.resolve_path(&[table])).await;

        match result {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    async fn create_table(&self, table: &str) -> Result<(), Self::Error> {
        fs::create_dir(self.resolve_path(&[table])).await?;

        Ok(())
    }

    async fn delete_table(&self, table: &str) -> Result<(), Self::Error> {
        if self.has_table(table).await? {
            fs::remove_dir_all(self.resolve_path(&[table])).await?;
        }

        Ok(())
    }

    async fn get_keys<I>(&self, table: &str) -> Result<I, Self::Error>
    where
        I: FromIterator<String>,
    {
        let mut stream = ReadDirStream::new(fs::read_dir(self.resolve_path(&[table])).await?);
        let mut output = Vec::new();

        while let Some(raw) = stream.next().await {
            let entry = raw?;

            if entry.file_type().await?.is_dir() {
                continue;
            }

            let filename = Self::resolve_key(entry.file_name()).ok();

            if filename.is_none() {
                continue;
            }

            output.push(unsafe { filename.unwrap_unchecked() });
        }

        Ok(output.into_iter().collect())
    }

    async fn get<D>(&self, table: &str, id: &str) -> Result<D, Self::Error>
    where
        D: for<'de> Deserialize<'de>,
    {
        let filename = id.to_owned() + ".json";
        let path = self.resolve_path(&[table, filename.as_str()]);
        let file: std::fs::File = fs::File::open(path).await?.into_std().await;
        let reader = io::BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    async fn has(&self, table: &str, id: &str) -> Result<bool, Self::Error> {
        let filename = id.to_owned() + ".json";
        let file = fs::read(self.resolve_path(&[table, filename.as_str()])).await;

        match file {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    async fn create<S>(&self, table: &str, id: &str, value: &S) -> Result<(), Self::Error>
    where
        S: Serialize + Send + Sync,
    {
        let filepath = id.to_owned() + ".json";

        let path = self.resolve_path(&[table, filepath.as_str()]);

        if self.has(table, id).await? {
            return Err(JsonError::FileAlreadyExists(path));
        }

        let serialized = serde_json::to_string(value)?.into_bytes();

        fs::write(path, serialized).await?;

        Ok(())
    }

    async fn update<S>(&self, table: &str, id: &str, value: &S) -> Result<(), Self::Error>
    where
        S: Serialize + Send + Sync,
    {
        let serialized = serde_json::to_string(value)?.into_bytes();

        let filepath = id.to_owned() + ".json";

        let path = self.resolve_path(&[table, filepath.as_str()]);

        fs::write(path, serialized).await?;

        Ok(())
    }

    async fn replace<S>(&self, table: &str, id: &str, value: &S) -> Result<(), Self::Error>
    where
        S: Serialize + Send + Sync,
    {
        self.update(table, id, value).await?;

        Ok(())
    }

    async fn delete(&self, table: &str, id: &str) -> Result<(), Self::Error> {
        let filename = id.to_owned() + ".json";

        fs::remove_file(self.resolve_path(&[table, filename.as_str()])).await?;

        Ok(())
    }
}
