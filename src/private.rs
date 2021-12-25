use crate::{
	action::{
		CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OpTarget, ReadOperation,
		TableTarget, UpdateOperation,
	},
	Action, Entry,
};

pub trait Sealed {}

impl Sealed for CreateOperation {}
impl Sealed for ReadOperation {}
impl Sealed for UpdateOperation {}
impl Sealed for DeleteOperation {}
impl Sealed for TableTarget {}
impl Sealed for EntryTarget {}
impl<S: Entry, C: CrudOperation, T: OpTarget> Sealed for Action<S, C, T> {}
