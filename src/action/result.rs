use std::{convert::Infallible, error::Error, fmt::Debug, iter::FromIterator};

#[derive(Debug, Clone,  PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use = "this `ActionResult` may be an `Error` variant, which should be handled"]
pub enum ActionResult<R, E> {
	Create,
	ReadSingle(Option<R>),
	ReadMultiple(Vec<R>),
	Update,
	Delete(bool),
	Error(E),
}

impl<R, E: Error> ActionResult<R, E> {
	pub fn create(self) -> Result<(), E> {
		match self {
			Self::Create => Ok(()),
			Self::Error(e) => Err(e),
			_ => panic!(
				"called `ActionResult::create` on a `{}` value",
				self.variant()
			),
		}
	}

	pub fn read_single(self) -> Result<Option<R>, E> {
		match self {
			Self::ReadSingle(v) => Ok(v),
			Self::Error(e) => Err(e),
			_ => panic!(
				"called `ActionResult::single_read` on a `{}` value",
				self.variant()
			),
		}
	}

	pub fn read_multiple<I: FromIterator<R>>(self) -> Result<I, E> {
		match self {
			Self::ReadMultiple(v) => Ok(v.into_iter().collect()),
			Self::Error(e) => Err(e),
			_ => panic!("called `ActionResult::multiple_read` on a `{}` value", self.variant())
		}
	}

	pub fn update(self) -> Result<(), E> {
		match self {
			Self::Update => Ok(()),
			Self::Error(e) => Err(e),
			_ => panic!(
				"called `ActionResult::update` on a `{}` value",
				self.variant()
			),
		}
	}

	pub fn delete(self) -> Result<bool, E> {
		match self {
			Self::Delete(b) => Ok(b),
			Self::Error(e) => Err(e),
			_ => panic!(
				"called `ActionResult::delete` on a `{}` value",
				self.variant()
			),
		}
	}

	fn variant(&self) -> &str {
		match self {
			Self::Create => "Create",
			Self::ReadSingle(_) | Self::ReadMultiple(_) => "Read",
			Self::Update => "Update",
			Self::Delete(_) => "Delete",
			Self::Error(_) => "Error",
		}
	}
}
