use starchart::{
	action::{
		ActionKind, CreateEntryAction, EntryTarget, ReadEntryAction, ReadOperation,
		ReadTableAction, TargetKind, UpdateEntryAction,
	},
	Action, Result,
};
use starchart_backends::fs::{
	transcoders::{BinaryTranscoder, JsonTranscoder, TomlTranscoder},
	FsBackend,
};

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
	let def = TestSettings::new(1);
	let mut action: Action<TestSettings, ReadOperation, EntryTarget> = Action::new();
	action.set_entry(&def);

	assert_eq!(action.data(), Some(&TestSettings::new(1)));
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
async fn basic_run() -> Result<()> {
	let _lock = TEST_GUARD.lock().await;
	let test_name = basic_run.test_name();
	let backend = FsBackend::new(TomlTranscoder::pretty(), "toml".to_owned(), OUT_DIR)?;
	let gateway = setup_chart(backend, &test_name).await;

	for i in 0..3 {
		let settings = TestSettings::new(i);

		let mut action: CreateEntryAction<TestSettings> = Action::new();
		action.set_table(&test_name).set_entry(&settings);

		action.run_create_entry(&gateway).await?;
	}

	let mut read_table: ReadTableAction<TestSettings> = Action::new();

	read_table.set_table(&test_name);

	let mut values: Vec<_> = read_table.run_read_table(&gateway).await?;

	let mut expected = (0..3).map(TestSettings::new).collect::<Vec<_>>();

	let cmp_fn = |a: &TestSettings, b: &TestSettings| a.id.cmp(&b.id);

	values.sort_by(cmp_fn);
	expected.sort_by(cmp_fn);

	assert_eq!(values, expected);

	Ok(())
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn duplicate_creates() -> Result<()> {
	let _lock = TEST_GUARD.lock().await;
	let test_name = duplicate_creates.test_name();
	let backend = FsBackend::new(JsonTranscoder::pretty(), "json".to_owned(), OUT_DIR)?;
	let gateway = setup_chart(backend, &test_name).await;
	let mut def = TestSettings::new(7);

	def.array.extend([6, 7, 8]);

	let mut create_action: CreateEntryAction<TestSettings> = Action::new();

	create_action.set_table(&test_name).set_entry(&def);

	let double_create = create_action.clone();

	assert!(create_action.run_create_entry(&gateway).await.is_ok());

	assert!(double_create.run_create_entry(&gateway).await.is_ok());

	Ok(())
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn read_and_update() -> Result<()> {
	let _lock = TEST_GUARD.lock().await;
	let test_name = read_and_update.test_name();
	let backend = FsBackend::new(BinaryTranscoder::bincode(), "bin".to_owned(), OUT_DIR)?;
	let gateway = setup_chart(backend, &test_name).await;

	{
		let def = TestSettings::new(1);
		let mut action: CreateEntryAction<TestSettings> = Action::new();
		action.set_table(&test_name).set_entry(&def);

		action.run_create_entry(&gateway).await?;
	}

	let mut read_action: ReadEntryAction<TestSettings> = Action::new();

	read_action.set_key(&1_u32).set_table(&test_name);

	let reread_action = read_action.clone();

	let value = read_action.run_read_entry(&gateway).await?;
	assert_eq!(value, Some(TestSettings::new(1)));

	let new_settings = TestSettings {
		id: 1,
		value: "goodbye!".to_owned(),
		array: vec![6, 7, 8],
		opt: None,
	};

	let mut update_action: UpdateEntryAction<TestSettings> = Action::new();

	update_action.set_table(&test_name).set_entry(&new_settings);

	update_action.run_update_entry(&gateway).await?;

	assert_eq!(
		reread_action.run_read_entry(&gateway).await?,
		Some(new_settings)
	);

	Ok(())
}
