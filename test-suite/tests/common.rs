use std::{
	any::type_name,
	cmp::Ordering,
	fs::remove_dir_all,
	io::ErrorKind,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use starchart::{action::CreateTableAction, backend::Backend, Action, IndexEntry, Starchart};
use tokio::sync::Mutex;

pub static TEST_GUARD: Mutex<()> = Mutex::const_new(());

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, IndexEntry)]
pub struct TestSettings {
	pub id: u32,
	pub value: String,
	pub array: Vec<u8>,
	pub opt: Option<f64>,
}

impl TestSettings {
	pub fn new(id: u32) -> Self {
		Self {
			id,
			value: "hello, world!".to_owned(),
			array: vec![1, 2, 3, 4, 5],
			opt: Some(4.2),
		}
	}

	pub fn key_sort(&self, other: &Self) -> Ordering {
		self.id.cmp(&other.id)
	}
}

#[derive(Debug)]
pub struct TestPath(PathBuf);

impl TestPath {
	pub fn new(test_name: &str) -> Self {
		let path = PathBuf::from(env!("OUT_DIR")).join(test_name);

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

impl AsRef<Path> for TestPath {
	fn as_ref(&self) -> &Path {
		self.0.as_ref()
	}
}

pub async fn setup_chart<T: Backend>(backend: T, table: &str) -> Starchart<T> {
	let chart = Starchart::new(backend).await.unwrap();

	let mut action: CreateTableAction<TestSettings> = Action::new();

	action.set_table(table);

	action.run_create_table(&chart).await.unwrap();

	chart
}

pub trait TestName {
	fn test_name(&self) -> String;
}

impl<T> TestName for T {
	fn test_name(&self) -> String {
		let name = type_name::<T>();

		if let Some(position) = name.rfind("::") {
			if let Some(slice) = name.get(position + 2..) {
				return slice.to_owned();
			}
		}

		name.to_owned()
	}
}
