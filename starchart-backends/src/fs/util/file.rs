use std::io::Error;
#[cfg(unix)]
use std::os::unix::prelude::*;

use tokio::fs;

#[derive(Debug)]
pub struct File(fs::File);

#[cfg(unix)]
impl AsRawFd for File {
	fn as_raw_fd(&self) -> RawFd {
		self.0.as_raw_fd()
	}
}

static mut IS_UNSUPPORTED: Option<bool> = None;

fn is_unsupported(err: &Error) -> bool {
	if let Some(v) = unsafe { IS_UNSUPPORTED } {
		v
	} else {
		let v = imp::is_unsupported(err);
		unsafe { IS_UNSUPPORTED = Some(v) };
		v
	}
}

mod imp {
	use std::{
		io::{Error, Result as IoResult},
		os::unix::io::AsRawFd,
	};

	pub fn lock_shared(file: &impl AsRawFd) -> IoResult<()> {
		flock(file, libc::LOCK_SH)
	}

	pub fn lock_exclusive(file: &impl AsRawFd) -> IoResult<()> {
		flock(file, libc::LOCK_EX)
	}

	pub fn unlock(file: &impl AsRawFd) -> IoResult<()> {
		flock(file, libc::LOCK_UN)
	}

	pub fn is_contended(err: &Error) -> bool {
		err.raw_os_error().map_or(false, |x| x == libc::EWOULDBLOCK)
	}

	#[allow(unreachable_patterns, clippy::match_same_arms)]
	pub fn is_unsupported(err: &Error) -> bool {
		match err.raw_os_error() {
			Some(libc::ENOTSUP | libc::EOPNOTSUPP) => true,
			#[cfg(target_os = "linux")]
			Some(libc::ENOSYS) => true,
			_ => false,
		}
	}

	#[cfg(not(target_os = "solaris"))]
	fn flock(file: &impl AsRawFd, flag: libc::c_int) -> IoResult<()> {
		let ret = unsafe { libc::flock(file.as_raw_fd(), flag) };

		if ret < 0 {
			Err(Error::last_os_error())
		} else {
			Ok(())
		}
	}

	#[cfg(target_os = "solaris")]
	fn flock(_: &impl AsRawFd, _: libc::c_int) -> IoResult<()> {
		Ok(())
	}
}
