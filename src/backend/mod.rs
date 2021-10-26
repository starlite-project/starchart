#[cfg(feature = "json")]
mod json;

#[cfg(feature = "json")]
pub use self::json::JsonBackend;

pub trait Backend {}
