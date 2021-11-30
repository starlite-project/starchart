use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use starchart::{
	action::{CreateEntryAction, CreateTableAction, ReadEntryAction},
	backend::JsonBackend,
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
async fn main() -> ChartResult<(), JsonBackend> {
	Ok(())
}
