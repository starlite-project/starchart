use std::iter::FromIterator;

use futures_util::FutureExt;

use super::AccessorError;
use crate::{backend::Backend, util::PinBoxFuture, Entry, Starchart};

pub trait EntryAccessor<'a>: Entry + 'a {
	fn read_entry<B: Backend>(
		chart: &'a Starchart<B>,
		table: &'a str,
		key: &'a str,
	) -> PinBoxFuture<'a, Result<Option<Self>, AccessorError>> {
		chart.access().read_entry(table, key).boxed()
	}

	fn read_table<B: Backend, I: FromIterator<Self> + Send + 'a>(
		chart: &'a Starchart<B>,
		table: &'a str,
	) -> PinBoxFuture<'a, Result<I, AccessorError>> {
		chart.access().read_table(table).boxed()
	}
}
