use std::{
	cmp::Ordering,
	sync::atomic::{AtomicU64, Ordering as SyncOrdering},
};

use serde::{Deserialize, Serialize};
use starchart::{
	action::{CreateEntryAction, CreateTableAction, ReadTableAction},
	backend::JsonBackend,
	Action, ChartResult, IndexEntry, Starchart,
};

static IDS: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, IndexEntry)]
struct Settings {
	id: u64,
	name: String,
	age: u8,
}

impl Settings {
	pub fn new(name: String, age: u8) -> Self {
		let id = IDS.fetch_add(1, SyncOrdering::SeqCst);

		Self { id, name, age }
	}
}

impl PartialOrd for Settings {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.id.partial_cmp(&other.id)
	}
}

impl Ord for Settings {
	fn cmp(&self, other: &Self) -> Ordering {
		self.id.cmp(&other.id)
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ChartResult<(), JsonBackend> {
	let chart = Starchart::new(JsonBackend::new("./examples/get_all_db")?).await?;

	{
		let mut action: CreateTableAction<Settings> = Action::new();
		action.set_table("foo".to_owned());

		// chart.run(action).await??;
		action.run(&chart).await?.unwrap_create();
	}

	// Insert some entries into the table.
	for (age, name) in vec![
		(21, "John Doe".to_owned()),
		(42, "Ferris".to_owned()),
		(73, "The Queen".to_owned()),
	] {
		let mut action: CreateEntryAction<Settings> = Action::new();
		action.set_table("foo".to_owned()).set_entry(&Settings::new(name, age));
		// chart.run(action).await??;
		action.run(&chart).await?.unwrap_create();
	}

	let mut read_all_action: ReadTableAction<Settings> = Action::new();
	read_all_action.set_table("foo".to_owned());

	// let mut values: Vec<Settings> = chart.run(read_all_action).await??;
	let mut values = read_all_action
		.run(&chart)
		.await?
		.unwrap_multi_read::<Vec<_>>();

	values.sort();

	assert_eq!(
		values,
		vec![
			Settings {
				id: 1,
				name: "John Doe".to_owned(),
				age: 21
			},
			Settings {
				id: 2,
				name: "Ferris".to_owned(),
				age: 42
			},
			Settings {
				id: 3,
				name: "The Queen".to_owned(),
				age: 73
			}
		]
	);

	Ok(())
}
