#![allow(clippy::missing_panics_doc, missing_docs)]

use std::{
	convert::Infallible,
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	iter::FromIterator,
};

use crate::Entry;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use = "an ActionResult should be asserted"]
pub enum ActionResult<R> {
	Create,
	ReadSingle(Option<R>),
	ReadMultiple(Vec<R>),
	Update,
	Delete(bool),
}

impl<R: Entry> ActionResult<R> {
	#[track_caller]
	#[inline]
	pub fn create(self) {
		assert!(
			matches!(self, Self::Create),
			"called `ActionResult::create` on a `{}` value",
			self
		);
	}

	#[track_caller]
	#[inline]
	pub fn read_single(self) -> Option<R> {
		if let Self::ReadSingle(v) = self {
			v
		} else {
			panic!("called `ActionResult::read_single` on a `{}` value", self);
		}
	}

	#[track_caller]
	#[inline]
	pub fn read_multiple<I: FromIterator<R>>(self) -> I {
		if let Self::ReadMultiple(v) = self {
			v.into_iter().collect()
		} else {
			panic!("called `ActionResult::read_multiple` on a `{}` value", self)
		}
	}

	#[track_caller]
	#[inline]
	pub fn update(self) {
		assert!(
			matches!(self, Self::Update),
			"called `ActionResult::update` on a `{}` value",
			self
		);
	}

	#[track_caller]
	#[inline]
	pub fn delete(self) -> bool {
		if let Self::Delete(b) = self {
			b
		} else {
			panic!("called `ActionResult::delete` on a `{}` value", self)
		}
	}
}

impl<R> Display for ActionResult<R> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Create => f.write_str("Create"),
			Self::ReadSingle(_) | Self::ReadMultiple(_) => f.write_str("Read"),
			Self::Update => f.write_str("Update"),
			Self::Delete(_) => f.write_str("Delete"),
		}
	}
}
