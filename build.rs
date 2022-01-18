use rustc_version::{version_meta, Channel, Error};

fn main() -> Result<(), Error> {
	let version_data = version_meta()?;
	let minor = version_data.semver.minor;
	if let Channel::Nightly = version_data.channel {
		println!("cargo:rustc-cfg=docsrs");
		// if env::var("CARGO_FEATURE_NIGHTLY").ok().is_some() {
		// 	println!("cargo:rustc-cfg=nighttime"); // for fun
		// }

		// if version_data.semver.minor >= 58 {
		// 	println!("cargo:rustc-cfg=unwrap_unchecked")
		// }
	}

	if minor < 58 {
		println!("cargo:rustc-cfg=no_unwrap_unchecked")
	}

	Ok(())
}
