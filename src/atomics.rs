use parking_lot::{lock_api::RawRwLock as IRawRwLock, RawRwLock};
use std::fmt::{Debug, Formatter, Result};

pub struct GuardState {
    inner: RawRwLock,
}

impl GuardState {
    pub const fn new() -> Self {
        Self {
            inner: RawRwLock::INIT,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }

    fn drop_shared(&self) {
        unsafe {
            self.inner.unlock_shared();
        }
    }

    fn drop_exclusive(&self) {
        unsafe {
            self.inner.unlock_exclusive();
        }
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
