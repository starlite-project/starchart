use std::{
	cmp::Ordering,
	sync::atomic::{AtomicU64, Ordering as SyncOrdering},
};

use serde::{Deserialize, Serialize};
use starchart::{
	action::{CreateEntryAction, CreateTableAction, ReadTableAction},
	backend::JsonBackend,
	Action, IndexEntry, Result, Starchart,
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
async fn main() -> Result<()> {
	let chart = Starchart::new(JsonBackend::new("./examples/get_all_db")?).await?;

	{
		let mut action: CreateTableAction<Settings> = Action::new();
		action.set_table("foo");

		action.run_create_table(&chart).await?;
	}

	// Insert some entries into the table.
	for (age, name) in vec![
		(21, "John Doe".to_owned()),
		(42, "Ferris".to_owned()),
		(73, "The Queen".to_owned()),
	] {
		let value = Settings::new(name, age);
		let mut action: CreateEntryAction<Settings> = Action::new();
		action
			.set_table("foo")
			.set_entry(&value);
		action.run_create_entry(&chart).await?;
	}

	let mut read_all_action: ReadTableAction<Settings> = Action::new();
	read_all_action.set_table("foo");

	let mut values = read_all_action
		.run_read_table::<JsonBackend, Vec<_>>(&chart)
		.await?;

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
