use std::fmt::{Display, Formatter, Result as FmtResult};

use rustc_version::{version_meta, Channel, Error};

// this is wholley unneeded but it makes things easier.
#[derive(Debug, Clone, Copy)]
enum CfgKeys {
	Docsrs,
	NoUnwrapUnchecked,
}

impl Display for CfgKeys {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Docsrs => f.write_str("docsrs"),
			Self::NoUnwrapUnchecked => f.write_str("no_unwrap_unchecked"),
		}
	}
}

fn main() -> Result<(), Error> {
	let version_data = version_meta()?;
	let minor = version_data.semver.minor;
	if let Channel::Nightly = version_data.channel {
		println!("cargo:rustc-cfg={}", CfgKeys::Docsrs);
	}

	if minor < 58 {
		println!("cargo:rustc-cfg={}", CfgKeys::NoUnwrapUnchecked);
	}

	Ok(())
}
