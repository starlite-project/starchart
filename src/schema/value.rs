#[derive(Debug, Clone)]
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
}

impl Drop for SchemaValue {
	fn drop(&mut self) {
		println!("dropping");
	}
}
