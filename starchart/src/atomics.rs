#![allow(clippy::non_send_fields_in_send_ty)]
use parking_lot::{lock_api::RawRwLock as _, RawRwLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct Guard(RwLock<()>);

impl Guard {
	pub const fn new() -> Self {
		Self(RwLock::const_new(RawRwLock::INIT, ()))
	}

	pub fn shared(&self) -> SharedGuard {
		let inner = self.0.read();

		SharedGuard(inner)
	}

	pub fn exclusive(&self) -> ExclusiveGuard {
		let inner = self.0.write();

		ExclusiveGuard(inner)
	}
}

impl Default for Guard {
	fn default() -> Self {
		Self::new()
	}
}

// implementing send doesn't matter bc we're not actually editing the value, just using it for a locking mechanism
pub struct SharedGuard<'a>(RwLockReadGuard<'a, ()>);

unsafe impl<'a> Send for SharedGuard<'a> {}

pub struct ExclusiveGuard<'a>(RwLockWriteGuard<'a, ()>);

unsafe impl<'a> Send for ExclusiveGuard<'a> {}
