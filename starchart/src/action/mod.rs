use std::{borrow::Cow, ops::Deref};

use crate::{Entry, IndexEntry, Key};

#[derive(Debug)]
pub struct Action<'v, D> {
	table: &'v str,
	key: Option<Cow<'static, str>>,
	data: Option<&'v D>,
}

impl<'v, D> Action<'v, D> {
	pub const fn new(table: &'v str) -> Self {
		Self {
			table,
			key: None,
			data: None,
		}
	}

	pub const fn table(&self) -> &str {
		self.table
	}

	pub fn key(&self) -> Option<&str> {
		self.key.as_deref()
	}
}

impl<'v, D: Entry> Action<'v, D> {
	pub fn data(&self) -> Option<&'v D> {
		self.data
	}

	#[must_use]
	pub fn entry(&self) -> Option<(&str, &'v D)> {
		match (self.key(), self.data()) {
			(Some(k), Some(v)) => Some((k, v)),
			_ => None,
		}
	}

	pub fn with_key<K: Key>(table: &'v str, key: &K) -> Self {
		let mut act = Self::new(table);

		act.set_key(key);

		act
	}

	pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
		self.key.replace(key.to_key());

		self
	}

	pub fn set_data(&mut self, entry: &'v D) -> &mut Self {
		self.data.replace(entry);

		self
	}
}

impl<'v, D: IndexEntry> Action<'v, D> {
	pub fn with_entry(table: &'v str, entry: &'v D) -> Self {
		let mut act = Self::new(table);

		act.set_entry(entry);

		act
	}

	pub fn set_entry(&mut self, entry: &'v D) -> &mut Self {
		self.set_key(entry.key()).set_data(entry)
	}
}

impl<'v, D> Clone for Action<'v, D> {
	fn clone(&self) -> Self {
		Self {
			table: self.table,
			key: self.key.clone(),
			data: self.data,
		}
	}
}

