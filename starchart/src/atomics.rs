//! Atomics that the chart uses to ensure synchronized data accesses.

use tokio::sync::{Semaphore, SemaphorePermit};

/// Maximum number of locks for a [`Guard`].
const MAX_LOCKS: usize = (u32::MAX >> 3) as usize;

/// A guard for preventing data races easily.
#[derive(Debug)]
pub struct Guard(Semaphore);

impl Guard {
	/// Creates a new [`Guard`].
	pub const fn new() -> Self {
		Self(Semaphore::const_new(MAX_LOCKS))
	}

	/// Gives a shared read lock.
	pub async fn read(&self) -> ReadLock<'_> {
		// SAFETY: this is okay because semaphore will only return an error if it's closed.
		ReadLock(unsafe { self.0.acquire().await.unwrap_unchecked() })
	}

	/// Gives an exclusive write lock.
	#[allow(clippy::cast_possible_truncation)]
	pub async fn write(&self) -> WriteLock<'_> {
		// SAFETY: this is okay because semaphore will only return an error if it's closed.
		WriteLock(unsafe {
			self.0
				.acquire_many(MAX_LOCKS as u32)
				.await
				.unwrap_unchecked()
		})
	}
}

impl Default for Guard {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for Guard {
	fn drop(&mut self) {
		self.0.close();
	}
}

/// Shareable read lock.
#[derive(Debug)]
pub struct ReadLock<'a>(SemaphorePermit<'a>);

/// Exclusive write lock.
#[derive(Debug)]
pub struct WriteLock<'a>(SemaphorePermit<'a>);
