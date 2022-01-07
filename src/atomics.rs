use parking_lot::{lock_api::RawRwLock as _, RawRwLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use send_wrapper::SendWrapper;

#[derive(Debug)]
pub struct Guard(RwLock<()>);

impl Guard {
	pub const fn new() -> Self {
		Self(RwLock::const_new(RawRwLock::INIT, ()))
	}

	pub fn shared(&self) -> SharedGuard {
		let inner = self.0.read();

		SharedGuard(SendWrapper::new(inner))
	}

	pub fn exclusive(&self) -> ExclusiveGuard {
		let inner = self.0.write();

		ExclusiveGuard(SendWrapper::new(inner))
	}
}

impl Default for Guard {
	fn default() -> Self {
		Self::new()
	}
}

pub struct SharedGuard<'a>(SendWrapper<RwLockReadGuard<'a, ()>>);

pub struct ExclusiveGuard<'a>(SendWrapper<RwLockWriteGuard<'a, ()>>);
