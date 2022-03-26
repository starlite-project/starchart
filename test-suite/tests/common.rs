use std::{
	io::ErrorKind,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use starchart::{Action, Backend, Error as ChartError, IndexEntry, Result, Starchart};

pub const OUT_DIR: &str = env!("OUT_DIR");

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, IndexEntry)]
pub struct TestSettings {
	pub id: u32,
	pub value: String,
	pub array: Vec<u8>,
	pub opt: Option<f64>,
}

impl TestSettings {
	pub const fn new(id: u32, value: String, array: Vec<u8>, opt: Option<f64>) -> Self {
		Self {
			id,
			value,
			array,
			opt,
		}
	}

	pub fn with_defaults(id: u32) -> Self {
		Self::new(
			id,
			"hello, world!".to_owned(),
			vec![1, 2, 3, 4, 5],
			Some(4.2),
		)
	}
}

pub async fn setup_gateway<B: Backend>(backend: B, table: &str) -> Result<Starchart<B>> {
	let chart = Starchart::new(backend)
		.await
		.map_err(|e| ChartError::backend(Box::new(e)))?;

	Action::<TestSettings>::new(table)
		.create_table(&chart)
		.await?;

	Ok(chart)
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct TestPath(PathBuf);

impl TestPath {
	pub fn new(test_name: &str) -> Self {
		let path = PathBuf::from(OUT_DIR).join(test_name);

		if let Err(e) = std::fs::remove_dir_all(&path) {
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
