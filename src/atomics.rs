//! Atomics crate used to allow for atomic operations on a database system.
//!
//! Anything within this module is a private implementation detail that can be changed at
//! any time, and should not be relied upon.
//!
//! In addition, breaking changes to this module will not be reflected in ``SemVer`` updates.

use std::{
	fmt::{Debug, Formatter, Result},
	sync::{
		atomic::{AtomicU8, Ordering},
		RwLock, RwLockReadGuard, RwLockWriteGuard,
	},
};

/// The kind of lock an [`AtomicGuard`] is holding.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum GuardKind {
	/// The [`AtomicGuard`] is unlocked.
	Unlocked = 0,
	/// The [`AtomicGuard`] is locked in a shared fashion.
	Shared = 1,
	/// The [`AtomicGuard`] is locked in an exclusive fashion.
	Exclusive = 2,
}

impl GuardKind {
	pub(crate) const fn as_u8(self) -> u8 {
		self as u8
	}
}

/// The mechanism used to allow multiple readers and only one writer
/// to access a shared database.
///
/// This uses [`parking_lot`]'s [`RawRwLock`] internally.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
/// # use starchart::atomics::AtomicGuard;
/// # use std::{sync::Arc, thread, time::Duration};
/// let guard = Arc::new(AtomicGuard::new());
///
/// let first_shared = guard.clone();
/// let second_shared = guard.clone();
/// let exclusive = guard.clone();
///
/// let first_shared = thread::spawn(move || {
///    let guard = first_shared.shared();
///
///     println!("doing work with the first shared lock");
///
///     /* ... */
///    # thread::sleep(Duration::from_millis(1000));
///
///     println!("done with the first lock");
/// });
///
/// let second_shared = thread::spawn(move || {
///     let guard = second_shared.shared();
///
///    println!("doing work with the second shared lock");
///
///    /* ... */
///     # thread::sleep(Duration::from_millis(750));
///
///    println!("done with the second lock");
/// });
///
/// let exclusive = thread::spawn(move || {
///     let guard = exclusive.exclusive();
///
///    println!("doing work with the exclusive lock");
///
///    /* ... */
///    # thread::sleep(Duration::from_millis(500));
///
///   println!("done with the exclusive lock");
/// });
///
/// first_shared.join()?;
/// second_shared.join()?;
/// exclusive.join()?;
///
/// # Ok(()) }
#[must_use = "a guard state does nothing if left unused"]
pub struct AtomicGuard {
	inner: RwLock<()>,
	kind: AtomicU8,
	amount: AtomicU8,
}

impl AtomicGuard {
	/// Creates a new, unlockecd [`AtomicGuard`].
	pub fn new() -> Self {
		Self {
			inner: RwLock::new(()),
			kind: AtomicU8::new(GuardKind::Unlocked.as_u8()),
			amount: AtomicU8::new(0),
		}
	}

	/// Checks whether the [`AtomicGuard`] is currently locked in any fashion.
	#[cfg(not(tarpaulin_include))]
	pub fn is_locked(&self) -> bool {
		self.kind() != GuardKind::Unlocked
	}

	/// Checks whether the [`AtomicGuard`] is currently locked exclusively.
	#[cfg(not(tarpaulin_include))]
	pub fn is_exclusive(&self) -> bool {
		self.kind() == GuardKind::Exclusive
	}

	/// Checks whether the [`AtomicGuard`] is currently locked shared.
	#[cfg(not(tarpaulin_include))]
	pub fn is_shared(&self) -> bool {
		self.kind() == GuardKind::Shared
	}

	/// The [`GuardKind`] this [`AtomicGuard`] is holding.
	///
	/// # Panics
	///
	/// Panics if the inner [`AtomicU8`] has been modified to be invalid.
	pub fn kind(&self) -> GuardKind {
		match self.kind.load(Ordering::SeqCst) {
			0 => GuardKind::Unlocked,
			1 => GuardKind::Shared,
			2 => GuardKind::Exclusive,
			_ => unreachable!(),
		}
	}

	/// Returns a [`SharedGuard`], allowing multiple locks to be acquired for shared reading.
	pub fn shared(&self) -> SharedGuard {
		let guard = self.inner.read().expect("poisoned guard");

		self.kind.store(GuardKind::Shared.as_u8(), Ordering::SeqCst);

		self.amount.fetch_add(1, Ordering::SeqCst);

		SharedGuard { state: self, guard }
	}

	/// Returns an [`ExclusiveGuard`], allowing only one lock to be acquired for exclusive writing.
	pub fn exclusive(&self) -> ExclusiveGuard {
		let guard = self.inner.write().expect("poisoned lock");

		self.kind
			.store(GuardKind::Exclusive.as_u8(), Ordering::SeqCst);

		self.amount.store(1, Ordering::SeqCst);

		ExclusiveGuard { state: self, guard }
	}

	fn drop_shared(&self) {
		self.amount.fetch_sub(1, Ordering::SeqCst);

		if self.amount.load(Ordering::SeqCst) == 0 {
			self.kind
				.store(GuardKind::Unlocked.as_u8(), Ordering::SeqCst);
		}
	}

	fn drop_exclusive(&self) {
		self.kind
			.store(GuardKind::Unlocked.as_u8(), Ordering::SeqCst);
		self.amount.store(0, Ordering::SeqCst);
	}
}

impl Default for AtomicGuard {
	fn default() -> Self {
		Self::new()
	}
}

impl Debug for AtomicGuard {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.debug_struct("AtomicGuard").finish()
	}
}

/// A shared guard for allowing multiple accesses to a resource.
pub struct SharedGuard<'a> {
	state: &'a AtomicGuard,
	guard: RwLockReadGuard<'a, ()>,
}

impl Drop for SharedGuard<'_> {
	fn drop(&mut self) {
		self.state.drop_shared();
	}
}

/// An exclusive guard for allowing only one access to a resource.
pub struct ExclusiveGuard<'a> {
	state: &'a AtomicGuard,
	guard: RwLockWriteGuard<'a, ()>,
}

impl Drop for ExclusiveGuard<'_> {
	fn drop(&mut self) {
		self.state.drop_exclusive();
	}
}

#[cfg(test)]
mod tests {
	use std::{fmt::Debug, sync::Arc, thread, time::Duration};

	use static_assertions::assert_impl_all;

	use super::AtomicGuard;

	assert_impl_all!(AtomicGuard: Debug, Default, Send, Sync);

	#[test]
	fn new_and_is_locked() {
		let state = AtomicGuard::new();

		assert!(!state.is_locked());

		let guard = state.shared();

		assert!(state.is_locked());

		drop(guard);

		assert!(!state.is_locked());

		let guard = state.exclusive();

		assert!(state.is_locked());

		drop(guard);

		assert!(!state.is_locked());
	}

	#[test]
	fn debug_and_default() {
		let state = AtomicGuard::default();

		let formatted = format!("{:?}", state);

		assert_eq!(formatted, "AtomicGuard");
	}

	// works with cargo test but not cargo tarpaulin, and only in CI
	// more investigation needed
	#[test]
	#[cfg_attr(tarpaulin, ignore)]
	fn guards() {
		let state = Arc::new(AtomicGuard::new());

		let first_shared = state.clone();
		let second_shared = state.clone();
		let exclusive = state.clone();

		let exclusive_thread = thread::spawn(move || {
			let _guard = exclusive.exclusive();

			thread::sleep(Duration::from_millis(500));

			assert!(exclusive.is_locked());
			assert!(!exclusive.is_shared());
			assert!(exclusive.is_exclusive());
		});

		let first_shared_thread = thread::spawn(move || {
			let _guard = first_shared.shared();

			thread::sleep(Duration::from_millis(1000));

			assert!(first_shared.is_locked());
			assert!(first_shared.is_shared());
			assert!(!first_shared.is_exclusive());
		});

		let second_shared_thread = thread::spawn(move || {
			let _guard = second_shared.shared();

			thread::sleep(Duration::from_millis(2000));

			assert!(second_shared.is_locked());
			assert!(second_shared.is_shared());
			assert!(!second_shared.is_exclusive());
		});

		first_shared_thread.join().unwrap();
		second_shared_thread.join().unwrap();
		exclusive_thread.join().unwrap();

		assert!(!state.is_locked());
	}
}
