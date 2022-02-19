use futures_util::{Future, FutureExt};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Default)]
pub struct Guard(RwLock<()>);

impl Guard {
	pub fn shared(&self) -> impl Future<Output = SharedGuard> {
		self.0.read().map(SharedGuard)
	}

	pub fn exclusive(&self) -> impl Future<Output = ExclusiveGuard> {
		self.0.write().map(ExclusiveGuard)
	}
}

pub struct SharedGuard<'a>(RwLockReadGuard<'a, ()>);

pub struct ExclusiveGuard<'a>(RwLockWriteGuard<'a, ()>);
