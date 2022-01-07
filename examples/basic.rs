use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use starchart::{
	action::{CreateEntryAction, CreateTableAction, ReadEntryAction},
	backend::MemoryBackend,
	Action, ChartResult, IndexEntry, Starchart,
};

static IDS: AtomicU64 = AtomicU64::new(1);

#[derive(
	Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, IndexEntry,
)]
struct Settings {
	id: u64,
	name: String,
	age: u8,
}

impl Settings {
	pub fn new(name: String, age: u8) -> Self {
		let id = IDS.fetch_add(1, Ordering::SeqCst);

		Self { id, name, age }
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ChartResult<(), MemoryBackend> {
	// Create an initialize the database.
	let chart = Starchart::new(MemoryBackend::default()).await?;

	// Create and run an action to create the "foo" table with the Settings struct.
	let mut create_table_action: CreateTableAction<Settings> = Action::new();

	create_table_action.set_table("foo");

	// chart.run(create_table_action).await??;
	create_table_action.run(&chart).await?.create();

	// Insert some entries into the table.
	for (age, name) in vec![
		(21, "John Doe".to_owned()),
		(42, "Ferris".to_owned()),
		(73, "The Queen".to_owned()),
	] {
		let mut action: CreateEntryAction<Settings> = Action::new();
		action.set_table("foo").set_entry(&Settings::new(name, age));
		// chart.run(action).await??;
		action.run(&chart).await?.create();
	}

	// Get a single entry.

	let the_queen = {
		// Action type helpers are named after their CRUD counterparts; Create, Read, Update, and Delete.
		let mut action: ReadEntryAction<Settings> = Action::new();
		action.set_key(&3_u64).set_table("foo");

		action
			.run(&chart)
			.await?
			.read_single()
			.expect("the queen has fallen!")
	};

	assert_eq!(
		the_queen,
		Settings {
			id: 3,
			name: "The Queen".to_owned(),
			age: 73
		}
	);

	Ok(())
}
