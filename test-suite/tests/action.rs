use starchart::{
	action::{
		ActionKind, CreateEntryAction, DeleteEntryAction, DeleteTableAction, EntryTarget,
		ReadEntryAction, ReadOperation, ReadTableAction, TargetKind, UpdateEntryAction,
	},
	Action, Result,
};
use starchart_backends::fs::{
	transcoders::{CborTranscoder, JsonTranscoder, TomlTranscoder, YamlTranscoder},
	FsBackend,
};

use self::common::*;

mod common;

#[test]
fn new_kind_and_target() {
	let action: ReadEntryAction<TestSettings> = Action::new("foo");

	assert_eq!(action.kind(), ActionKind::Read);
	assert_eq!(action.target(), TargetKind::Entry);
}

#[test]
fn set_methods() {
	let def = TestSettings::new(1);
	let mut action: Action<TestSettings, ReadOperation, EntryTarget> = Action::new("bar");
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

	assert_ne!(action.data(), Some(&def));
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
	let mut action: Action<TestSettings, ReadOperation, EntryTarget> = Action::new("table");

	assert!(action.validate_entry().is_err());
	action.set_entry(&def);
	assert!(action.validate_entry().is_ok());

	assert!(action.validate_table().is_ok());

	action.set_key(&"__metadata__");
	assert!(action.validate_key().is_err());

	action = Action::new("__metadata__");
	assert!(action.validate_table().is_err());
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn basic_run() -> Result<()> {
	let _lock = TEST_GUARD.lock().await;
	let test_name = basic_run.test_name();
	let backend = FsBackend::new(TomlTranscoder::pretty(), TestPath::new(&test_name))?;
	let gateway = setup_chart(backend, &test_name).await;

	for i in 0..3 {
		let settings = TestSettings::new(i);

		CreateEntryAction::with_entry(&test_name, &settings)
			.run_create_entry(&gateway)
			.await?;
	}

	let read_table: ReadTableAction<TestSettings> = Action::new(&test_name);

	let mut values: Vec<_> = read_table.run_read_table(&gateway).await?;

	let mut expected = (0..3).map(TestSettings::new).collect::<Vec<_>>();

	values.sort_by(|a, b| a.key_sort(b));
	expected.sort_by(|a, b| a.key_sort(b));

	assert_eq!(values, expected);

	Ok(())
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn duplicate_creates() -> Result<()> {
	let _lock = TEST_GUARD.lock().await;
	let test_name = duplicate_creates.test_name();
	let backend = FsBackend::new(JsonTranscoder::pretty(), TestPath::new(&test_name))?;
	let gateway = setup_chart(backend, &test_name).await;
	let mut def = TestSettings::new(7);

	def.array.extend([6, 7, 8]);

	let mut create_action: CreateEntryAction<TestSettings> = Action::new(&test_name);

	create_action.set_entry(&def);

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
	let backend = FsBackend::new(CborTranscoder::new(), TestPath::new(&test_name))?;
	let gateway = setup_chart(backend, &test_name).await;

	CreateEntryAction::with_entry(&test_name, &TestSettings::new(1))
		.run_create_entry(&gateway)
		.await?;

	let mut read_action: ReadEntryAction<TestSettings> = Action::new(&test_name);

	read_action.set_key(&1_u32);

	let reread_action = read_action.clone();

	let value = read_action.run_read_entry(&gateway).await?;
	assert_eq!(value, Some(TestSettings::new(1)));

	let new_settings = TestSettings {
		id: 1,
		value: "goodbye!".to_owned(),
		array: vec![6, 7, 8],
		opt: None,
	};

	let mut update_action: UpdateEntryAction<TestSettings> = Action::new(&test_name);

	update_action.set_entry(&new_settings);

	update_action.run_update_entry(&gateway).await?;

	assert_eq!(
		reread_action.run_read_entry(&gateway).await?,
		Some(new_settings)
	);

	Ok(())
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn deletes() -> Result<()> {
	let _lock = TEST_GUARD.lock().await;
	let test_name = deletes.test_name();
	let backend = FsBackend::new(YamlTranscoder::new(), TestPath::new(&test_name))?;
	let gateway = setup_chart(backend, &test_name).await;

	CreateEntryAction::with_entry(&test_name, &TestSettings::new(1))
		.run_create_entry(&gateway)
		.await?;

	let mut delete_action: DeleteEntryAction<TestSettings> = Action::new(&test_name);
	delete_action.set_key(&1_u32);
	assert!(delete_action.run_delete_entry(&gateway).await?);
	let mut read_action: ReadEntryAction<TestSettings> = Action::new(&test_name);
	read_action.set_key(&1_u32);
	assert_eq!(read_action.run_read_entry(&gateway).await?, None);

	let delete_table_action: DeleteTableAction<TestSettings> = Action::new(&test_name);
	assert!(delete_table_action.run_delete_table(&gateway).await?);
	let read_table: ReadTableAction<TestSettings> = Action::new(&test_name);

	let res: Result<Vec<_>> = read_table
		.run_read_table(&gateway)
		.await
		.map_err(Into::into);

	assert!(res.is_err());

	Ok(())
}
