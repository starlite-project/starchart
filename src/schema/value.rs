use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use serde_value::Value;

use super::{SchemaError, SchemaMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "a SchemaValue must be inserted into the SchemaMap"]
pub enum SchemaValue {
	Bool {
		default: bool,
	},
	U8 {
		minimum: u8,
		maximum: u8,
		default: u8,
	},
	U16 {
		minimum: u16,
		maximum: u16,
		default: u16,
	},
	U32 {
		minimum: u32,
		maximum: u32,
		default: u32,
	},
	U64 {
		minimum: u64,
		maximum: u64,
		default: u64,
	},
	I8 {
		minimum: i8,
		maximum: i8,
		default: i8,
	},
	I16 {
		minimum: i16,
		maximum: i16,
		default: i16,
	},
	I32 {
		minimum: i32,
		maximum: i32,
		default: i32,
	},
	I64 {
		minimum: i64,
		maximum: i64,
		default: i64,
	},
	F32 {
		minimum: f32,
		maximum: f32,
		default: f32,
	},
	F64 {
		minimum: f64,
		maximum: f64,
		default: f64,
	},
	Char {
		default: char,
	},
	String,
	Vec(Box<SchemaValue>),
	Option(Box<SchemaValue>),
	Subfolder(Box<SchemaMap>),
	Any,
}

impl SchemaValue {
	pub fn bool(default: Option<bool>) -> Self {
		Self::Bool {
			default: default.unwrap_or_default(),
		}
	}

	pub fn u8(minimum: Option<u8>, maximum: Option<u8>, default: Option<u8>) -> Self {
		Self::U8 {
			minimum: minimum.unwrap_or(u8::MIN),
			maximum: maximum.unwrap_or(u8::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn u16(minimum: Option<u16>, maximum: Option<u16>, default: Option<u16>) -> Self {
		Self::U16 {
			minimum: minimum.unwrap_or(u16::MIN),
			maximum: maximum.unwrap_or(u16::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn u32(minimum: Option<u32>, maximum: Option<u32>, default: Option<u32>) -> Self {
		Self::U32 {
			minimum: minimum.unwrap_or(u32::MIN),
			maximum: maximum.unwrap_or(u32::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn u64(minimum: Option<u64>, maximum: Option<u64>, default: Option<u64>) -> Self {
		Self::U64 {
			minimum: minimum.unwrap_or(u64::MIN),
			maximum: maximum.unwrap_or(u64::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn i8(minimum: Option<i8>, maximum: Option<i8>, default: Option<i8>) -> Self {
		Self::I8 {
			minimum: minimum.unwrap_or(i8::MIN),
			maximum: maximum.unwrap_or(i8::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn i16(minimum: Option<i16>, maximum: Option<i16>, default: Option<i16>) -> Self {
		Self::I16 {
			minimum: minimum.unwrap_or(i16::MIN),
			maximum: maximum.unwrap_or(i16::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn i32(minimum: Option<i32>, maximum: Option<i32>, default: Option<i32>) -> Self {
		Self::I32 {
			minimum: minimum.unwrap_or(i32::MIN),
			maximum: maximum.unwrap_or(i32::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn i64(minimum: Option<i64>, maximum: Option<i64>, default: Option<i64>) -> Self {
		Self::I64 {
			minimum: minimum.unwrap_or(i64::MIN),
			maximum: maximum.unwrap_or(i64::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn f32(minimum: Option<f32>, maximum: Option<f32>, default: Option<f32>) -> Self {
		Self::F32 {
			minimum: minimum.unwrap_or(f32::MIN),
			maximum: maximum.unwrap_or(f32::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn f64(minimum: Option<f64>, maximum: Option<f64>, default: Option<f64>) -> Self {
		Self::F64 {
			minimum: minimum.unwrap_or(f64::MIN),
			maximum: maximum.unwrap_or(f64::MAX),
			default: default.unwrap_or_default(),
		}
	}

	pub fn char(default: Option<char>) -> Self {
		Self::Char {
			default: default.unwrap_or_default(),
		}
	}

	pub const fn string() -> Self {
		Self::String
	}

	pub const fn any() -> Self {
		Self::Any
	}

	pub fn vector(inner: Self) -> Self {
		Self::Vec(Box::new(inner))
	}

	pub fn option(inner: Self) -> Self {
		Self::Option(Box::new(inner))
	}

	pub fn subfolder<F: FnOnce(SchemaMap) -> Result<SchemaMap, SchemaError>>(
		closure: F,
	) -> Result<Self, SchemaError> {
		let map = closure(SchemaMap::new())?;

		Ok(Self::Subfolder(Box::new(map)))
	}
}

impl TryFrom<Value> for SchemaValue {
	type Error = SchemaError;

	fn try_from(raw: Value) -> Result<Self, Self::Error> {
		Ok(match raw {
			Value::Bool(_) => Self::bool(None),
			Value::U8(_) => Self::u8(None, None, None),
			Value::U16(_) => Self::u16(None, None, None),
			Value::U32(_) => Self::u32(None, None, None),
			Value::U64(_) => Self::u64(None, None, None),
			Value::I8(_) => Self::i8(None, None, None),
			Value::I16(_) => Self::i16(None, None, None),
			Value::I32(_) => Self::i32(None, None, None),
			Value::I64(_) => Self::i64(None, None, None),
			Value::F32(_) => Self::f32(None, None, None),
			Value::F64(_) => Self::f64(None, None, None),
			Value::Char(_) => Self::char(None),
			Value::String(_) => Self::string(),
			Value::Unit | Value::Bytes(_) => return Err(SchemaError::UnsupportedValue),
			Value::Option(opt) => Self::option({
				if let Some(kind) = opt {
					Self::try_from(*kind)?
				} else {
					Self::any()
				}
			}),
			Value::Newtype(v) => Self::try_from(*v)?,
			Value::Seq(v) => Self::vector({
				if v.is_empty() || v.len() > 1 {
					Self::any()
				} else {
					let first = v.first().cloned();
					match first {
						Some(val) => Self::try_from(val)?,
						None => Self::any(),
					}
				}
			}),
			Value::Map(map) => Self::subfolder(move |mut schema_map| {
				for (raw_key, raw_value) in map {
					let key = raw_key.deserialize_into().map_err(|_| SchemaError::UnsupportedKeyType)?;

					schema_map = schema_map.include(key, Self::try_from(raw_value)?)?;
				}

				Ok(schema_map)
			})?,
		})
	}
}
