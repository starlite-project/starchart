//! Atomics crate used to allow for atomic operations on a database system.
//!
//! Anything within this module is a private implementation detail that can be changed at
//! any time, and should not be relied upon.
//!
//! In addition, breaking changes to this module will not be reflected in SemVer updates.

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    fmt::{Debug, Formatter, Result},
    sync::atomic::{AtomicUsize, Ordering},
};

/// The mechanism used to allow multiple readers and only one writer
/// to access a shared database.
///
/// This uses [`parking_lot`]'s [`RwLock`] internally.
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
    kind: AtomicUsize,
    amount: AtomicUsize,
}

impl AtomicGuard {
    /// Creates a new, unlockecd [`AtomicGuard`].
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(()),
            kind: AtomicUsize::new(0),
            amount: AtomicUsize::new(0),
        }
    }

    /// Checks whether the [`AtomicGuard`] is currently locked in any fashion.
    pub fn is_locked(&self) -> bool {
        self.kind.load(Ordering::SeqCst) != 0
    }

    /// Checks whether the [`AtomicGuard`] is currently locked exclusively.
    // #[cfg(not(tarpaulin_include))]
    pub fn is_exclusive(&self) -> bool {
        self.kind.load(Ordering::SeqCst) == 2
    }

    /// Checks whether the [`AtomicGuard`] is currently locked shared.
    pub fn is_shared(&self) -> bool {
        self.kind.load(Ordering::SeqCst) == 1
    }

    /// Returns a [`SharedGuard`], allowing multiple locks to be acquired for shared reading.
    pub fn shared(&self) -> SharedGuard {
        let read_guard = self.inner.read();

        self.kind.store(1, Ordering::SeqCst);

        self.amount.fetch_add(1, Ordering::SeqCst);

        SharedGuard {
            state: read_guard,
            atomic_guard: self,
        }
    }

    /// Returns an [`ExclusiveGuard`], allowing only one lock to be acquired for exclusive writing.
    pub fn exclusive(&self) -> ExclusiveGuard {
        let write_guard = self.inner.write();

        self.kind.store(2, Ordering::SeqCst);

        self.amount.store(1, Ordering::SeqCst);

        ExclusiveGuard {
            state: write_guard,
            atomic_guard: self,
        }
    }

    fn drop_shared(&self) {
        self.amount.fetch_sub(1, Ordering::SeqCst);

        if self.amount.load(Ordering::SeqCst) == 0 {
            self.kind.store(0, Ordering::SeqCst);
        }
    }

    fn drop_exclusive(&self) {
        self.kind.store(0, Ordering::SeqCst);
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
    state: RwLockReadGuard<'a, ()>,
    atomic_guard: &'a AtomicGuard,
}

impl Drop for SharedGuard<'_> {
    fn drop(&mut self) {
        self.atomic_guard.drop_shared();
    }
}

/// An exclusive guard for allowing only one access to a resource.
pub struct ExclusiveGuard<'a> {
    state: RwLockWriteGuard<'a, ()>,
    atomic_guard: &'a AtomicGuard,
}

impl Drop for ExclusiveGuard<'_> {
    fn drop(&mut self) {
        self.atomic_guard.drop_exclusive();
    }
}

#[cfg(test)]
mod tests {
    use super::AtomicGuard;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, sync::Arc, thread, time::Duration};

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

    #[test]
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
