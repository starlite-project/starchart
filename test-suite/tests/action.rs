use starchart::{
	action::{ActionKind, EntryTarget, ReadEntryAction, ReadOperation, TargetKind},
	Action,
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
