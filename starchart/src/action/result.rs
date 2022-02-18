use std::{
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	hint::unreachable_unchecked,
	iter::FromIterator,
};

use crate::Entry;

/// A custom [`Result`] type that allows the [`run`] method to
/// return multiple different types.
///
/// [`run`]: crate::action::DynamicAction::run
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use = "an ActionResult should be asserted"]
pub enum ActionResult<R> {
	/// Indicates a [`CreateOperation`] was performed.
	///
	/// [`CreateOperation`]: crate::action::CreateOperation
	Create,
	/// Indicates a [`ReadOperation`] was performed on an [`EntryTarget`].
	///
	/// This value can be retrieved with [`Self::unwrap_single_read`].
	///
	/// [`ReadOperation`]: crate::action::ReadOperation
	/// [`EntryTarget`]: crate::action::EntryTarget
	SingleRead(Option<R>),
	/// Indicates a [`ReadOperation`] was performed on a [`TableTarget`].
	///
	/// This value can be retrieved with [`Self::unwrap_multi_read`].
	///
	/// [`ReadOperation`]: crate::action::ReadOperation
	/// [`TableTarget`]: crate::action::TableTarget
	MultiRead(Vec<R>),
	/// Indicates an [`UpdateOperation`] was performed.
	///
	/// [`UpdateOperation`]: crate::action::UpdateOperation
	Update,
	/// Indicates a [`DeleteOperation`] was performed.
	///
	/// The inner value can be retrieved with [`Self::unwrap_delete`] to determine whether the delete was successful.
	///
	/// [`DeleteOperation`]: crate::action::DeleteOperation
	Delete(bool),
}

impl<R> ActionResult<R> {
	/// Returns [`true`] if the result is [`Create`].
	///
	/// [`Create`]: Self::Create
	pub const fn is_create(&self) -> bool {
		matches!(self, Self::Create)
	}

	/// Returns [`true`] if the result is [`SingleRead`].
	///
	/// [`SingleRead`]: Self::SingleRead
	pub const fn is_single_read(&self) -> bool {
		matches!(self, Self::SingleRead(_))
	}

	/// Returns [`true`] if the result is [`MultiRead`].
	///
	/// [`MultiRead`]: Self::MultiRead
	pub const fn is_multi_read(&self) -> bool {
		matches!(self, Self::MultiRead(_))
	}

	/// Returns [`true`] if the result is either a [`SingleRead`] or a [`MultiRead`].
	///
	/// [`SingleRead`]: Self::SingleRead
	/// [`MultiRead`]: Self::MultiRead
	pub const fn is_read(&self) -> bool {
		self.is_single_read() || self.is_multi_read()
	}

	/// Returns [`true`] if the result is an [`Update`].
	///
	/// [`Update`]: Self::Update
	pub const fn is_update(&self) -> bool {
		matches!(self, Self::Update)
	}

	/// Returns [`true`] if the result is a [`Delete`].
	///
	/// [`Delete`]: Self::Delete
	pub const fn is_delete(&self) -> bool {
		matches!(self, Self::Delete(_))
	}
}

impl<R: Entry> ActionResult<R> {
	/// Unwraps the [`ActionResult`] and asserts it's a create
	///
	/// # Panics
	///
	/// This method panics if [`Self`] is anything other than [`Create`].
	///
	/// [`Create`]: Self::Create
	#[track_caller]
	#[inline]
	pub fn unwrap_create(self) {
		assert!(
			self.is_create(),
			"called `ActionResult::create` on a `{}` value",
			self
		);
	}

	/// Unwraps the [`ActionResult::Create`] value, without checking if the value is any other type.
	///
	/// # Safety
	///
	/// Calling this method on anything other than a [`Create`] will cause `UB`.
	///
	/// [`Create`]: Self::Create
	#[inline]
	#[track_caller]
	pub unsafe fn unwrap_create_unchecked(self) {
		debug_assert!(self.is_create());
		if let Self::Create = self {
		} else {
			unreachable_unchecked()
		}
	}

	/// Unwraps the [`ActionResult`] and asserts it's a single read.
	///
	/// # Panics
	///
	/// This method panics if [`Self`] is anything other than [`SingleRead`].
	///
	/// [`SingleRead`]: Self::SingleRead
	#[track_caller]
	#[inline]
	pub fn unwrap_single_read(self) -> Option<R> {
		if let Self::SingleRead(v) = self {
			v
		} else {
			panic!("called `ActionResult::single_read` on a `{}` value", self);
		}
	}

	/// Unwraps the [`ActionResult::SingleRead`] value, without checking if the value is any other type.
	///
	/// # Safety
	///
	/// Calling this method on anything other than a [`SingleRead`] will cause `UB`.
	///
	/// [`SingleRead`]: Self::SingleRead
	#[track_caller]
	#[inline]
	pub unsafe fn unwrap_single_read_unchecked(self) -> Option<R> {
		debug_assert!(self.is_single_read());
		if let Self::SingleRead(v) = self {
			v
		} else {
			unreachable_unchecked()
		}
	}

	/// Unwraps the [`ActionResult`] and asserts it's a multi-read.
	///
	/// # Panics
	///
	/// This method panics if [`Self`] is anything other than [`MultiRead`].
	///
	/// [`MultiRead`]: Self::MultiRead
	#[track_caller]
	#[inline]
	pub fn unwrap_multi_read<I: FromIterator<R>>(self) -> I {
		if let Self::MultiRead(v) = self {
			v.into_iter().collect()
		} else {
			panic!("called `ActionResult::multi_read` on a `{}` value", self)
		}
	}

	/// Unwraps the [`ActionResult::MultiRead`] value, without checking if the value is any other type.
	///
	/// # Safety
	///
	/// Calling this method on anything other than a [`MultiRead`] will cause `UB`.
	///
	/// [`MultiRead`]: Self::MultiRead
	pub unsafe fn unwrap_multi_read_unchecked<I: FromIterator<R>>(self) -> I {
		debug_assert!(self.is_multi_read());
		if let Self::MultiRead(v) = self {
			v.into_iter().collect()
		} else {
			unreachable_unchecked()
		}
	}

	/// Unwraps the [`ActionResult`] and asserts it's an update.
	///
	/// # Panics
	///
	/// This method panics if [`Self`] is anything other than [`Update`].
	///
	/// [`Update`]: Self::Update
	#[track_caller]
	#[inline]
	pub fn unwrap_update(self) {
		assert!(
			self.is_update(),
			"called `ActionResult::update` on a `{}` value",
			self
		);
	}

	/// Unwraps the [`ActionResult::Update`] value, without checking if the value is any other type.
	///
	/// # Safety
	///
	/// Calling this method on anything other than an [`Update`] will cause `UB`.
	///
	/// [`Update`]: Self::Update
	pub unsafe fn unwrap_update_unchecked(self) {
		debug_assert!(self.is_update());
		if let Self::Update = self {
		} else {
			unreachable_unchecked()
		}
	}

	/// Unwraps the [`ActionResult`] and asserts it's a delete.
	///
	/// # Panics
	///
	/// This method panics if [`Self`] is anything other than [`Delete`].
	///
	/// [`Delete`]: Self::Delete
	#[track_caller]
	#[inline]
	pub fn unwrap_delete(self) -> bool {
		if let Self::Delete(b) = self {
			b
		} else {
			panic!("called `ActionResult::delete` on a `{}` value", self)
		}
	}

	/// Unwraps the [`ActionResult::Delete`] value, without checking if the value is any other type.
	///
	/// # Safety
	///
	/// Calling this method on anything other than a [`Delete`] will cause `UB`.
	///
	/// [`Delete`]: Self::Delete
	pub unsafe fn unwrap_delete_unchecked(self) -> bool {
		debug_assert!(self.is_delete());
		if let Self::Delete(b) = self {
			b
		} else {
			unreachable_unchecked()
		}
	}
}

impl<R> Display for ActionResult<R> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Create => f.write_str("Create"),
			Self::SingleRead(_) | Self::MultiRead(_) => f.write_str("Read"),
			Self::Update => f.write_str("Update"),
			Self::Delete(_) => f.write_str("Delete"),
		}
	}
}
