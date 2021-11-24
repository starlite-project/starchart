use std::hint::unreachable_unchecked;

pub unsafe trait InnerUnwrap<T> {
	unsafe fn inner_unwrap(self) -> T;
}

unsafe impl<T> InnerUnwrap<T> for Option<T> {
	#[inline]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		debug_assert!(self.is_some());
		if let Some(v) = self {
			v
		} else {
			unreachable_unchecked()
		}
	}
}

unsafe impl<T, E> InnerUnwrap<T> for Result<T, E> {
	#[inline]
	#[track_caller]
	unsafe fn inner_unwrap(self) -> T {
		debug_assert!(self.is_ok());
		if let Ok(v) = self {
			v
		} else {
			unreachable_unchecked()
		}
	}
}
