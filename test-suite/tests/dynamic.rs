use starchart::{
	action::{ActionKind, DynamicAction, TargetKind},
	Result,
};
use starchart_backends::fs::{transcoders::TomlTranscoder, FsBackend};

use self::common::*;

mod common;

#[test]
fn new_kind_and_target() {
	let action: DynamicAction<TestSettings> =
		DynamicAction::new("foo".to_owned(), ActionKind::Read, TargetKind::Entry);

	assert_eq!(action.kind(), ActionKind::Read);
	assert_eq!(action.target(), TargetKind::Entry);
}

#[test]
fn set_methods() {
	let mut action = DynamicAction::new("bar".to_owned(), ActionKind::Read, TargetKind::Entry);
	action.set_entry(TestSettings::new(1));

	assert_eq!(action.data(), Some(&TestSettings::new(1)));
	assert_eq!(action.key(), Some("1"));
	assert_eq!(action.table(), "bar");

	let mut action = action.set_key(&"2").clone();
	assert_eq!(action.key(), Some("2"));

	let changed = TestSettings {
		id: 1,
		value: "goodbye".to_owned(),
		array: vec![5, 4, 3],
		opt: None,
	};

	let action = action.set_data(changed);

	assert_ne!(action.data(), Some(&TestSettings::new(1)));

	action
		.set_kind(ActionKind::Create)
		.set_target(TargetKind::Table);
	assert_eq!(action.kind(), ActionKind::Create);
	assert_eq!(action.target(), TargetKind::Table);
}

#[test]
fn default() {
	let default: DynamicAction<TestSettings> = DynamicAction::default();

	assert!(default.data().is_none());
	assert!(default.key().is_none());
}

#[test]
fn validation_methods() {
	let mut action = DynamicAction::new("table".to_owned(), ActionKind::Read, TargetKind::Entry);

	assert!(action.validate_entry().is_err());
	action.set_entry(TestSettings::default());
	assert!(action.validate_entry().is_ok());

	assert!(action.validate_table().is_ok());

	action.set_key(&"__metadata__");
	assert!(action.validate_key().is_err());

	action = DynamicAction::new(
		"__metadata__".to_owned(),
		ActionKind::Read,
		TargetKind::Entry,
	);
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

		DynamicAction::with_entry(
			test_name.clone(),
			ActionKind::Create,
			TargetKind::Entry,
			settings,
		)
		.run(&gateway)
		.await?
		.unwrap_create();
	}

	Ok(())
}
