use rustc_version::{version_meta, Channel, Error};

fn main() -> Result<(), Error> {
	if let Channel::Nightly = version_meta()?.channel {
		println!("cargo:rustc-cfg=docsrs");
	}

	Ok(())
}
