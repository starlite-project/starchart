//! Atomics crate used to allow for atomic operations on a database system.
//!
//! Anything within this module is a private implementation detail that can be changed at
//! any time, and should not be relied upon.

use parking_lot::{lock_api::RawRwLock as IRawRwLock, RawRwLock};
use std::{
    fmt::{Debug, Formatter, Result},
    sync::atomic::{AtomicU8, Ordering},
};

/// The type of Lock the [`GuardState`] is holding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GuardKind {
    /// The state is unlocked.
    Unlocked,
    /// The state is locked with a shared lock, and multiple can be acquired for shared reading.
    Shared,
    /// The state is locked with an exclusive lock, and cannot be read from or written to while this lock is active.
    Exclusive,
}

impl GuardKind {
    /// Casts the [`GuardKind`] to a [`prim@u8`].
    ///
    /// This is used to allow it to be held in an [`AtomicU8`].
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Converts the value from a [`prim@u8`].
    ///
    /// # Panics
    ///
    /// This panics if the value is not `0`, `1`, or `2`.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Unlocked,
            1 => Self::Shared,
            2 => Self::Exclusive,
            _ => unreachable!(),
        }
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
/// # use starchart::atomics::GuardState;
/// # use std::{sync::Arc, thread, time::Duration};
/// let guard = Arc::new(GuardState::new());
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
pub struct GuardState {
    inner: RawRwLock,
    kind: AtomicU8,
}

impl GuardState {
    /// Creates a new, unlockecd [`GuardState`].
    pub const fn new() -> Self {
        Self {
            inner: RawRwLock::INIT,
            kind: AtomicU8::new(GuardKind::Unlocked.as_u8()),
        }
    }

    /// Checks whether the [`GuardState`] is currently locked in any fashion.
    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }

    /// Checks whether the [`GuardState`] is currently locked exclusively.
    pub fn is_exclusive(&self) -> bool {
        matches!(self.kind(), GuardKind::Exclusive)
    }

    /// Checks whether the [`GuardState`] is currently locked in a shared fashion.
    pub fn is_shared(&self) -> bool {
        matches!(self.kind(), GuardKind::Shared)
    }

    /// Returns the [`GuardKind`] of the current state.
    pub fn kind(&self) -> GuardKind {
        GuardKind::from_u8(self.kind.load(Ordering::Relaxed))
    }

    /// Returns a [`SharedGuard`], allowing multiple locks to be acquired for shared reading.
    pub fn shared(&self) -> SharedGuard {
        self.inner.lock_shared();
        self.kind
            .store(GuardKind::Shared.as_u8(), Ordering::Relaxed);
        SharedGuard { state: self }
    }

    /// Returns an [`ExclusiveGuard`], allowing only one lock to be acquired for exclusive writing.
    pub fn exclusive(&self) -> ExclusiveGuard {
        self.inner.lock_exclusive();
        self.kind
            .store(GuardKind::Exclusive.as_u8(), Ordering::Relaxed);
        ExclusiveGuard { state: self }
    }

    fn drop_shared(&self) {
        unsafe {
            self.inner.unlock_shared();
        }
        if !self.is_locked() {
            self.kind
                .store(GuardKind::Unlocked.as_u8(), Ordering::Relaxed);
        }
    }

    fn drop_exclusive(&self) {
        unsafe {
            self.inner.unlock_exclusive();
        }
        self.kind
            .store(GuardKind::Unlocked.as_u8(), Ordering::Relaxed);
    }
}

impl Default for GuardState {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for GuardState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("GuardState").finish()
    }
}

/// A shared guard for allowing multiple accesses to a resource.
pub struct SharedGuard<'a> {
    state: &'a GuardState,
}

impl<'a> Drop for SharedGuard<'a> {
    fn drop(&mut self) {
        self.state.drop_shared();
    }
}

/// An exclusive guard for allowing only one access to a resource.
pub struct ExclusiveGuard<'a> {
    state: &'a GuardState,
}

impl<'a> Drop for ExclusiveGuard<'a> {
    fn drop(&mut self) {
        self.state.drop_exclusive();
    }
}

#[cfg(test)]
mod tests {
    use super::GuardState;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, sync::Arc, thread, time::Duration};

    assert_impl_all!(GuardState: Debug, Default, Send, Sync);

    #[test]
    fn new_and_is_locked() {
        let state = GuardState::new();

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
        let state = GuardState::default();

        let formatted = format!("{:?}", state);

        assert_eq!(formatted, "GuardState");
    }

    #[cfg(not(tarpaulin))]
    #[test]
    fn guards() {
        let state = Arc::new(GuardState::new());

        let first_shared = state.clone();
        let second_shared = state.clone();
        let exclusive = state.clone();

        let first_shared_thread = thread::spawn(move || {
            let _guard = first_shared.shared();

            thread::sleep(Duration::from_millis(100));

            assert!(first_shared.is_locked());
            assert!(first_shared.is_shared());
            assert!(!first_shared.is_exclusive());
        });

        let second_shared_thread = thread::spawn(move || {
            let _guard = second_shared.shared();

            thread::sleep(Duration::from_millis(100));

            assert!(second_shared.is_locked());
            assert!(second_shared.is_shared());
            assert!(!second_shared.is_exclusive());
        });

        let exclusive_thread = thread::spawn(move || {
            let _guard = exclusive.exclusive();

            thread::sleep(Duration::from_millis(100));

            assert!(exclusive.is_locked());
            assert!(!exclusive.is_shared());
            assert!(exclusive.is_exclusive());
        });

        first_shared_thread.join().unwrap();
        second_shared_thread.join().unwrap();
        exclusive_thread.join().unwrap();

        assert!(!state.is_locked());
    }
}
