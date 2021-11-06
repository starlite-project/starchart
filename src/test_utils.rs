use std::future::Future;
use futures::executor::block_on;

pub trait SyncFuture: Future + Sized {
    fn wait(self) -> Self::Output;
}

impl<T: Future> SyncFuture for T {
    fn wait(self) -> T::Output {
        block_on(self)
    }
}