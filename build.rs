use rustc_version::{version_meta, Channel, Error};

fn main() -> Result<(), Error> {
	let version_data = version_meta()?;
	let minor = version_data.semver.minor;
	if let Channel::Nightly = version_data.channel {
		println!("cargo:rustc-cfg=docsrs");
	}

	if minor < 58 {
		println!("cargo:rustc-cfg=no_unwrap_unchecked")
	}

	Ok(())
}
