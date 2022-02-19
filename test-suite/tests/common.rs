use serde::{Deserialize, Serialize};
use starchart::{action::CreateTableAction, backend::Backend, Action, IndexEntry, Starchart};

pub const OUT_DIR: &str = env!("OUT_DIR");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IndexEntry)]
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

pub async fn setup_chart<T: Backend>(backend: T, with_table: bool) -> Starchart<T> {
	let chart = Starchart::new(backend).await.unwrap();
	if with_table {
		let mut action: CreateTableAction<TestSettings> = Action::new();

		action.set_table("table");

		action.run_create_table(&chart).await.unwrap();
	}

	chart
}
