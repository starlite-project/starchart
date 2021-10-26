#![feature(doc_cfg)]

pub mod backend;

use self::backend::Backend;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Database<B: Backend> {
    backend: B,
}