use std::{
	collections::VecDeque,
	fmt::{Display, Formatter, Result as FmtResult, Write},
	marker::PhantomData,
	str::FromStr,
};

use serde::{
	de::{Error as DeError, MapAccess, SeqAccess, Visitor},
	ser::SerializeStruct,
	Deserialize, Deserializer, Serialize, Serializer,
};

use super::{
	ActionError, ActionKind, ActionResult, ActionValidationError, ActionValidationErrorType,
	CreateOperation, CrudOperation, DeleteOperation, EntryTarget, InnerAction, OperationTarget,
	ReadOperation, TableTarget, TargetKind, UpdateOperation,
};
#[cfg(feature = "metadata")]
use crate::METADATA_KEY;
use crate::{backend::Backend, util::InnerUnwrap, Action, Entry, IndexEntry, Key, Starchart};

/// A dynamic [`Action`] for when certain parameters aren't known until runtime.
#[derive(Clone)]
#[must_use = "an action alone has no side effects"]
pub struct DynamicAction<S: ?Sized> {
	pub(super) key: Option<String>,
	pub(super) data: Option<Box<S>>,
	pub(super) table: Option<String>,
	pub(super) kind: ActionKind,
	pub(super) target: TargetKind,
}

impl<S: ?Sized> DynamicAction<S> {
	/// Creates a new action of the specified type and target.
	pub const fn new(kind: ActionKind, target: TargetKind) -> Self {
		Self {
			key: None,
			data: None,
			table: None,
			kind,
			target,
		}
	}

	/// Get a reference to the currently set key.
	#[must_use]
	pub fn key(&self) -> Option<&str> {
		self.key.as_deref()
	}

	/// Get a reference to the currently set table.
	#[must_use]
	pub fn table(&self) -> Option<&str> {
		self.table.as_deref()
	}

	/// Get the type of action.
	pub const fn kind(&self) -> ActionKind {
		self.kind
	}

	/// Get the current target of the action.
	pub const fn target(&self) -> TargetKind {
		self.target
	}
}

impl<S: Entry + ?Sized> DynamicAction<S> {
	/// Get a reference to the currently set data.
	#[must_use]
	pub fn data(&self) -> Option<&S> {
		self.data.as_deref()
	}

	/// Sets the table for this action.
	pub fn set_table(&mut self, table: String) -> &mut Self {
		self.table.replace(table);

		self
	}

	/// Sets the key for the action.
	///
	/// Users should prefer to call [`Self::set_entry`] over this, as setting the
	/// entry will automatically call this.
	///
	/// This is unused on [`TargetKind::Table`] actions.
	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.key.replace(key.to_key());

		self
	}

	/// Sets the data for the action.
	///
	/// This is unused on [`TargetKind::Table`] actions.
	pub fn set_data(&mut self, data: S) -> &mut Self {
		self.data.replace(Box::new(data));

		self
	}

	/// Sets the type of action we're performing.
	pub fn set_kind(&mut self, kind: ActionKind) -> &mut Self {
		self.kind = kind;

		self
	}

	/// Sets the target of the action we're performing.
	pub fn set_target(&mut self, target: TargetKind) -> &mut Self {
		self.target = target;

		self
	}

	/// Validate that the key has been set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_key`] has not yet been called.
	pub fn validate_key(&self) -> Result<(), ActionValidationError> {
		if self.key.is_none() {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Key,
			});
		}

		self.validate_metadata(self.key.as_deref())?;

		Ok(())
	}

	/// Validates that the table key is set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_table`] has not yet been called.
	pub fn validate_table(&self) -> Result<(), ActionValidationError> {
		if self.table.is_none() {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Table,
			});
		}

		self.validate_metadata(self.table.as_deref())?;

		Ok(())
	}

	/// Validates that the data has been set.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_data`] has not yet been called.
	pub fn validate_data(&self) -> Result<(), ActionValidationError> {
		if self.data.is_none() {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Data,
			});
		}

		Ok(())
	}

	/// Validates that both the key and data have been set.
	///
	/// # Errors
	///
	/// This errors if both the [`Self::set_key`] and [`Self::set_data`] (or [`Self::set_entry`]) has not been called.
	pub fn validate_entry(&self) -> Result<(), ActionValidationError> {
		self.validate_key()?;
		self.validate_data()
	}

	/// Validates that the key is not the private metadata key.
	///
	/// Does nothing if the `metadata` feature is not enabled.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_key`] was passed the private metadata key.
	#[cfg(feature = "metadata")]
	#[allow(clippy::unused_self)]
	pub fn validate_metadata(&self, key: Option<&str>) -> Result<(), ActionValidationError> {
		if key == Some(METADATA_KEY) {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Metadata,
			});
		}

		Ok(())
	}

	/// Validates that the key is not the private metadata key.
	///
	/// Does nothing if the `metadata` feature is not enabled.
	///
	/// # Errors
	///
	/// Errors if [`Self::set_key`] was passed the private metadata key.
	#[cfg(not(feature = "metadata"))]
	#[allow(clippy::unused_self)]
	pub fn validate_metadata(&self, _: Option<&str>) -> Result<(), ActionValidationError> {
		Ok(())
	}

	/// Runs an action to completion.
	///
	/// # Panics
	///
	/// This panics if the action kind is Update and the target is table, as updating tables is unsupported.
	///
	/// # Errors
	///
	/// This will raise an error if any of the static run methods in [`Action`] fail, as it uses those internally.
	pub async fn run<B: Backend>(
		self,
		chart: &Starchart<B>,
	) -> Result<ActionResult<S>, ActionError> {
		match (self.kind(), self.target()) {
			(ActionKind::Create, TargetKind::Entry) => {
				let stat = self.as_static::<CreateOperation, EntryTarget>()?;
				stat.run_create_entry(chart).await?;
				Ok(ActionResult::Create)
			}
			(ActionKind::Read, TargetKind::Entry) => {
				let stat = self.as_static::<ReadOperation, EntryTarget>()?;
				let ret = stat.run_read_entry(chart).await?;
				Ok(ActionResult::SingleRead(ret))
			}
			(ActionKind::Update, TargetKind::Entry) => {
				let stat = self.as_static::<UpdateOperation, EntryTarget>()?;
				stat.run_update_entry(chart).await?;
				Ok(ActionResult::Update)
			}
			(ActionKind::Delete, TargetKind::Entry) => {
				let stat = self.as_static::<DeleteOperation, EntryTarget>()?;
				let ret = stat.run_delete_entry(chart).await?;
				Ok(ActionResult::Delete(ret))
			}
			(ActionKind::Create, TargetKind::Table) => {
				let stat = self.as_static::<CreateOperation, TableTarget>()?;
				stat.run_create_table(chart).await?;
				Ok(ActionResult::Create)
			}
			(ActionKind::Read, TargetKind::Table) => {
				let stat = self.as_static::<ReadOperation, TableTarget>()?;
				let ret = stat.run_read_table(chart).await?;
				Ok(ActionResult::MultiRead(ret))
			}
			(ActionKind::Update, TargetKind::Table) => panic!("updating tables is unsupported"),
			(ActionKind::Delete, TargetKind::Table) => {
				let stat = self.as_static::<DeleteOperation, TableTarget>()?;
				let ret = stat.run_delete_table(chart).await?;
				Ok(ActionResult::Delete(ret))
			}
		}
	}

	/// Get a reference to a static action to run
	///
	/// # Errors
	///
	/// This will return an error if the generic parameters do not match the kind and target set for the dynamic action.
	pub fn as_static<C: CrudOperation, T: OperationTarget>(
		&self,
	) -> Result<Action<'_, S, C, T>, ActionValidationError> {
		if C::kind() != self.kind() || T::target() != self.target() {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Conversion,
			});
		}
		Ok(Action {
			inner: InnerAction {
				data: self.data.as_deref(),
				key: self.key.clone(),
				table: self.table.as_deref(),
			},
			kind: PhantomData,
			target: PhantomData,
		})
	}
}

impl<S: IndexEntry + ?Sized> DynamicAction<S> {
	/// Sets both a key and a value to run the action with.
	pub fn set_entry(&mut self, entry: S) -> &mut Self {
		self.set_key(&entry.key()).set_entry(entry)
	}
}

impl<E: ?Sized> Display for DynamicAction<E> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self.kind, f)?;
		f.write_char('.')?;
		Display::fmt(&self.target, f)?;
		if let Some(table) = &self.table {
			f.write_char('.')?;
			Display::fmt(&table, f)
		} else {
			Ok(())
		}
	}
}

impl<E: ?Sized> FromStr for DynamicAction<E> {
	type Err = ActionValidationError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut sections = s.split('.').collect::<VecDeque<_>>();
		if !(2..4).contains(&sections.len()) {
			return Err(ActionValidationError {
				source: None,
				kind: ActionValidationErrorType::Parse,
			});
		}

		let (kind, target, table) = unsafe {
			(
				sections.pop_front().inner_unwrap(),
				sections.pop_front().inner_unwrap(),
				sections.pop_front(),
			)
		};

		Ok(Self {
			key: None,
			data: None,
			table: table.map(ToOwned::to_owned),
			kind: kind.parse()?,
			target: target.parse()?,
		})
	}
}

impl<E: ?Sized> Serialize for DynamicAction<E> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		if serializer.is_human_readable() {
			self.to_string().serialize(serializer)
		} else {
			let mut state = serializer.serialize_struct("DynamicAction", 3)?;

			state.serialize_field("type", &self.kind)?;
			state.serialize_field("target", &self.target)?;
			state.serialize_field("table", &self.table.as_ref())?;

			state.end()
		}
	}
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum ActionField {
	Type,
	Target,
	Table,
}

struct ActionVisitor<S: ?Sized>(PhantomData<S>);

impl<S: ?Sized> Default for ActionVisitor<S> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<'de, S: ?Sized> Visitor<'de> for ActionVisitor<S> {
	type Value = DynamicAction<S>;

	fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
		formatter.write_str("a valid Action")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let kind = seq
			.next_element()?
			.ok_or_else(|| DeError::invalid_length(0, &self))?;
		let target = seq
			.next_element()?
			.ok_or_else(|| DeError::invalid_length(1, &self))?;
		let table = seq
			.next_element()?
			.ok_or_else(|| DeError::invalid_length(2, &self))?;

		Ok(DynamicAction {
			key: None,
			data: None,
			kind,
			target,
			table,
		})
	}

	fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
	where
		A: MapAccess<'de>,
	{
		let mut kind = None;
		let mut target = None;
		let mut table = None;

		while let Some(key) = map.next_key()? {
			match key {
				ActionField::Table => {
					if table.is_some() {
						return Err(DeError::duplicate_field("table"));
					}
					table = Some(map.next_value()?);
				}
				ActionField::Target => {
					if target.is_some() {
						return Err(DeError::duplicate_field("target"));
					}
					target = Some(map.next_value()?);
				}
				ActionField::Type => {
					if kind.is_some() {
						return Err(DeError::duplicate_field("type"));
					}
					kind = Some(map.next_value()?);
				}
			}
		}

		let kind = kind.ok_or_else(|| DeError::missing_field("type"))?;
		let target = target.ok_or_else(|| DeError::missing_field("target"))?;
		let table = table.ok_or_else(|| DeError::missing_field("table"))?;

		Ok(DynamicAction {
			table,
			kind,
			target,
			key: None,
			data: None,
		})
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		v.parse().map_err(DeError::custom)
	}
}

impl<'de, S> Deserialize<'de> for DynamicAction<S> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_struct(
			"DynamicAction",
			&["type", "target", "table"],
			ActionVisitor::default(),
		)
	}
}

// TODO: serde tests
// bincode data: [0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 102, 111, 111]
