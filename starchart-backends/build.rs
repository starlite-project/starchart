use std::fmt::{Display, Formatter, Result as FmtResult};

use rustc_version::{version_meta, Channel, Error};

// this is wholley unneeded but it makes things easier.
#[derive(Debug, Clone, Copy)]
enum CfgKeys {
	Docsrs,
}

impl Display for CfgKeys {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Docsrs => f.write_str("docsrs"),
		}
	}
}

fn main() -> Result<(), Error> {
	let version_data = version_meta()?;
	if let Channel::Nightly = version_data.channel {
		println!("cargo:rustc-cfg={}", CfgKeys::Docsrs);
	}
	Ok(())
}
