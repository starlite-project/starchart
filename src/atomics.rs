#![allow(missing_docs)]

use parking_lot::{lock_api::RawRwLock as IRawRwLock, RawRwLock};
use std::{
    fmt::{Debug, Formatter, Result},
    sync::atomic::{AtomicU8, Ordering},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GuardKind {
    Unlocked,
    Shared,
    Exclusive,
}

impl GuardKind {
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

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

#[must_use = "a guard state does nothing if left unused"]
pub struct GuardState {
    inner: RawRwLock,
    kind: AtomicU8,
}

impl GuardState {
    pub const fn new() -> Self {
        Self {
            inner: RawRwLock::INIT,
            kind: AtomicU8::new(GuardKind::Unlocked.as_u8()),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }

    pub fn is_exclusive(&self) -> bool {
        matches!(self.kind(), GuardKind::Exclusive)
    }

    pub fn is_shared(&self) -> bool {
        matches!(self.kind(), GuardKind::Shared)
    }

    pub fn kind(&self) -> GuardKind {
        GuardKind::from_u8(self.kind.load(Ordering::Relaxed))
    }

    pub fn shared(&self) -> SharedGuard {
        self.inner.lock_shared();
        self.kind
            .store(GuardKind::Shared.as_u8(), Ordering::Relaxed);
        SharedGuard { state: self }
    }

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

pub struct SharedGuard<'a> {
    state: &'a GuardState,
}

impl<'a> Drop for SharedGuard<'a> {
    fn drop(&mut self) {
        self.state.drop_shared();
    }
}

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
