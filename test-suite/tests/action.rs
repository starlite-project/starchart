use std::collections::HashMap;

use starchart::{Action, Result};
use starchart_backends::fs::{
	transcoders::{CborTranscoder, JsonTranscoder, TomlTranscoder, YamlTranscoder},
	FsBackend,
};

use self::common::*;

mod common;

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn basic_run() -> Result<()> {
	let test_name = "basic_run".to_owned();
	let backend = FsBackend::new(TomlTranscoder::pretty(), OUT_DIR)?;
	let gateway = setup_gateway(backend, &test_name).await?;

	for i in 1..=3 {
		let settings = TestSettings::new(
			i.into(),
			format!("iteration {}", i),
			(0..i).into_iter().collect(),
			Some(i as f64),
		);

		let mut action = Action::new(&test_name);
		action.set_entry(&settings).create_entry(&gateway).await?;
	}

	let read_table = Action::<TestSettings>::new(&test_name);

	let mut values = read_table
		.read_table::<HashMap<_, _>, _>(&gateway)
		.await?
		.into_iter()
		.map(|(_, v)| v)
		.collect::<Vec<_>>();

	let expected = vec![
		TestSettings {
			id: 1,
			value: "iteration 1".to_owned(),
			array: vec![0],
			opt: Some(1.0),
		},
		TestSettings {
			id: 2,
			value: "iteration 2".to_owned(),
			array: vec![0, 1],
			opt: Some(2.0),
		},
		TestSettings {
			id: 3,
			value: "iteration 3".to_owned(),
			array: vec![0, 1, 2],
			opt: Some(3.0),
		},
	];

	values.sort_by(|a, b| a.id.cmp(&b.id));

	assert_eq!(values, expected);

	Ok(())
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn duplicate_creates() -> Result<()> {
	let test_name = "duplicate_creates".to_owned();
	let backend = FsBackend::new(JsonTranscoder::pretty(), OUT_DIR)?;
	let gateway = setup_gateway(backend, &test_name).await?;

	let def = TestSettings::with_defaults(7);

	let create_action = Action::with_entry(&test_name, &def);

	let second_create = create_action.clone();

	assert!(create_action.create_entry(&gateway).await.is_ok());
	assert!(second_create.create_entry(&gateway).await.is_ok());

	Ok(())
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn read_and_update() -> Result<()> {
	let test_name = "read_and_update".to_owned();
	let backend = FsBackend::new(CborTranscoder::new(), OUT_DIR)?;
	let gateway = setup_gateway(backend, &test_name).await?;

	{
		let def = TestSettings::with_defaults(1);
		Action::with_entry(&test_name, &def)
			.create_entry(&gateway)
			.await?;
	}

	let read_action = Action::with_key(&test_name, &1_u32);

	let reread_action = read_action.clone();

	let value = read_action.read_entry(&gateway).await?;
	assert_eq!(value, Some(TestSettings::with_defaults(1)));

	let new_settings = TestSettings::new(1, "goodbye!".to_owned(), vec![6, 7, 8], None);

	Action::with_entry(&test_name, &new_settings)
		.update_entry(&gateway)
		.await?;

	assert_eq!(
		reread_action.read_entry(&gateway).await?,
		Some(new_settings)
	);

	Ok(())
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn deletes() -> Result<()> {
	let test_name = "deletes".to_owned();
	let backend = FsBackend::new(YamlTranscoder::new(), OUT_DIR)?;
	let gateway = setup_gateway(backend, &test_name).await?;

	let def = TestSettings::with_defaults(1);

	Action::with_entry(&test_name, &def)
		.create_entry(&gateway)
		.await?;

	let delete_action = Action::<TestSettings>::with_key(&test_name, &1_u32);
	assert!(delete_action.delete_entry(&gateway).await?);
	let read_action = Action::<TestSettings>::with_key(&test_name, &1_u32);
	assert_eq!(read_action.read_entry(&gateway).await?, None);

	let delete_table_action = Action::<TestSettings>::new(&test_name);
	assert!(delete_table_action.delete_table(&gateway).await?);
	let read_table = Action::<TestSettings>::new(&test_name);

	let res: Result<HashMap<_, _>> = read_table.read_table(&gateway).await.map_err(Into::into);

	assert!(res.is_err());

	Ok(())
}