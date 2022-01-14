use std::env;

use rustc_version::{version_meta, Channel, Error};

fn main() -> Result<(), Error> {
	let version_data = version_meta()?;
	if let Channel::Nightly = version_data.channel {
		println!("cargo:rustc-cfg=docsrs");
		if env::var("CARGO_FEATURE_NIGHTLY").ok().is_some() {
			println!("cargo:rustc-cfg=nighttime"); // for fun
		}

		if version_data.semver.minor >= 58 {
			println!("cargo:rustc-cfg=unwrap_unchecked")
		}
	}

	Ok(())
}
