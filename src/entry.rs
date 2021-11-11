#![allow(clippy::empty_enum)]

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Debug;

// /// The key trait to be implemented on [`Entity`] to allow an easy way to get keys.
// pub trait Key {
//     /// The method to transform a [`Key`] into a value.
//     fn to_key(&self) -> String;
// }

/// A marker trait for use within the [`Database`].
///
/// This signifies that the type can be stored within a [`Database`].
///
/// [`Database`]: crate::Database
pub trait Entity: Serialize + DeserializeOwned + Debug + Send + Sync {
    /// Returns a valid key for the database to index from.
    fn to_key(&self) -> String;
}

/// An entity with no variants, and therefore cannot exist.
/// 
/// This is useful for [`ActionResult`] where you know it's not an [`ActionKind::Read`].
/// 
/// [`ActionResult`]: crate::action::result::ActionResult
/// [`ActionKind::Read`]: crate::action::ActionKind::Read
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NeverEntity {}

impl Entity for NeverEntity {
    fn to_key(&self) -> String {
        unimplemented!()
    }
}