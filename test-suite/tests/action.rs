use starchart::{Action, Error, action::{ActionKind, CreateEntryAction, EntryTarget, ReadEntryAction, ReadOperation, TargetKind}};
use starchart_backends::fs::{FsBackend, transcoders::TomlTranscoder};

use self::common::*;

mod common;

#[test]
fn new_kind_and_target() {
	let action: ReadEntryAction<TestSettings> = Action::new();

	assert_eq!(action.kind(), ActionKind::Read);
	assert_eq!(action.target(), TargetKind::Entry);
}

#[test]
fn set_methods() {
	let def = TestSettings::default();
	let mut action: Action<TestSettings, ReadOperation, EntryTarget> = Action::new();
	action.set_entry(&def);

	assert_eq!(action.data(), Some(&TestSettings::default()));
	assert_eq!(action.key(), Some("1"));

	let mut action = action.set_key(&"2").clone();
	assert_eq!(action.key(), Some("2"));

	let changed = TestSettings {
		id: 1,
		value: "goodbye!".to_owned(),
		array: vec![5, 4, 3],
		opt: None,
	};

	let action = action.set_data(&changed);

	assert_eq!(action.data(), Some(&changed));
}

#[test]
fn default() {
	let default = Action::<TestSettings, ReadOperation, EntryTarget>::default();

	assert!(default.data().is_none());
	assert!(default.key().is_none());
}

#[test]
fn validation_methods() {
	let def = TestSettings::default();
	let mut action: Action<TestSettings, ReadOperation, EntryTarget> = Action::new();

    assert!(action.validate_entry().is_err());
    action.set_entry(&def);
    assert!(action.validate_entry().is_ok());

    assert!(action.validate_table().is_err());
    action.set_table("table");
    assert!(action.validate_table().is_ok());

    action.set_key(&"__metadata__");
    assert!(action.validate_key().is_err());

    action.set_table("__metadata__");
    assert!(action.validate_table().is_err());
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn basic_run() -> Result<(), Error> {
	let _lock = TEST_GUARD.lock().await;
	let backend = FsBackend::new(TomlTranscoder::pretty(), "toml".to_owned(), OUT_DIR)?;
	let gateway = setup_chart(backend, true).await;

	for i in 0..3 {
		let settings = TestSettings {
			id: i,
			..TestSettings::default()
		};

		let mut action: CreateEntryAction<TestSettings> = Action::new();
		action.set_table("table").set_entry(&settings);

		action.run_create_entry(&gateway).await?;
	}

	Ok(())
}
