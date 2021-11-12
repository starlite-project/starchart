use super::ActionKind;

#[derive(Debug, Clone, Copy)]
pub struct CreateOperation;

#[derive(Debug, Clone, Copy)]
pub struct ReadOperation;

#[derive(Debug, Clone, Copy)]
pub struct UpdateOperation;

#[derive(Debug, Clone, Copy)]
pub struct DeleteOperation;

pub trait CrudOperation: private::Sealed {
	fn kind() -> ActionKind;
}

impl CrudOperation for CreateOperation {
	fn kind() -> ActionKind {
		ActionKind::Create
	}
}

mod private {
	use super::{CreateOperation, DeleteOperation, ReadOperation, UpdateOperation};

	pub trait Sealed {}

	impl Sealed for CreateOperation {}
	impl Sealed for ReadOperation {}
	impl Sealed for UpdateOperation {}
	impl Sealed for DeleteOperation {}
}
