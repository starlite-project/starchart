//! The backend that fetches and provides data for the [`Starchart`].
//!
//! [`Starchart`]: crate::Starchart

use std::{error::Error as StdError, iter::FromIterator};

use futures_util::{
	future::{join_all, ok, ready},
	FutureExt,
};

use self::futures::{
	CreateFuture, CreateTableFuture, DeleteFuture, DeleteTableFuture, EnsureFuture,
	EnsureTableFuture, GetAllFuture, GetFuture, HasFuture, HasTableFuture, InitFuture,
	ShutdownFuture, UpdateFuture,
};
use crate::Entry;

pub mod futures;

/// The backend to be used to manage data.
pub trait Backend: Send + Sync {
	/// The [`Error`] type that the backend will report up.
	///
	/// [`Error`]: std::error::Error
	type Error: Send + Sync + StdError + 'static;

	/// An optional initialization function, useful for making connections to the database.
	///
	/// The default impl does nothing
	fn init(&self) -> InitFuture<'_, Self::Error> {
		ok(()).boxed()
	}

	/// An optional shutdown function, useful for disconnecting from databases gracefully.
	///
	/// The default impl does nothing
	///
	/// # Safety
	///
	/// This should not fail, as it's ran upon dropping the [`Starchart`],
	/// and panicking during a drop means resources haven't adequately been cleaned up,
	/// which isn't inherintly UB however it should still be documented.
	///
	/// [`Starchart`]: crate::Starchart
	unsafe fn shutdown(&self) -> ShutdownFuture {
		ready(()).boxed()
	}

	/// Check if a table exists.
	fn has_table<'a>(&'a self, table: &'a str) -> HasTableFuture<'a, Self::Error>;

	/// Inserts or creates a table.
	fn create_table<'a>(&'a self, table: &'a str) -> CreateTableFuture<'a, Self::Error>;

	/// Deletes or drops a table.
	fn delete_table<'a>(&'a self, table: &'a str) -> DeleteTableFuture<'a, Self::Error>;

	/// Ensures a table exists.
	/// Uses [`Self::has_table`] first, then [`Self::create_table`] if it returns false.
	fn ensure_table<'a>(&'a self, table: &'a str) -> EnsureTableFuture<'a, Self::Error> {
		async move {
			if !self.has_table(table).await? {
				self.create_table(table).await?;
			}

			Ok(())
		}
		.boxed()
	}

	/// Gets all entries that match a predicate, to get all entries, use [`get_keys`] first.
	///
	/// [`get_keys`]: Self::get_keys
	fn get_all<'a, D, I>(&'a self, table: &'a str) -> GetAllFuture<'a, I, Self::Error>
	where
		D: Entry,
		I: FromIterator<(String, D)>;

	/// Gets a certain entry from a table.
	fn get<'a, D>(&'a self, table: &'a str, id: &'a str) -> GetFuture<'a, D, Self::Error>
	where
		D: Entry;

	/// Checks if an entry exists in a table.
	fn has<'a>(&'a self, table: &'a str, id: &'a str) -> HasFuture<'a, Self::Error>;

	/// Inserts a new entry into a table.
	fn create<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> CreateFuture<'a, Self::Error>
	where
		S: Entry;

	/// Ensures a value exists in the table.
	fn ensure<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> EnsureFuture<'a, Self::Error>
	where
		S: Entry,
	{
		async move {
			if !self.has(table, id).await? {
				self.create(table, id, value).await?;
			}

			Ok(())
		}
		.boxed()
	}

	/// Updates an existing entry in a table.
	fn update<'a, S>(
		&'a self,
		table: &'a str,
		id: &'a str,
		value: &'a S,
	) -> UpdateFuture<'a, Self::Error>
	where
		S: Entry;

	// We pass the generic value for things that need
	// type information, like the FsBackend, which uses it to
	// properly reencode the data.
	/// Deletes an entry from a table.
	fn delete<'a>(&'a self, table: &'a str, id: &'a str) -> DeleteFuture<'a, Self::Error>;
}
