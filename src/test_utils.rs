use futures::executor::block_on;
use std::future::Future;

pub trait SyncFuture: Future + Sized {
	fn wait(self) -> Self::Output;
}

impl<T: Future> SyncFuture for T {
	fn wait(self) -> T::Output {
		block_on(self)
	}
}
