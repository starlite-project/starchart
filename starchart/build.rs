use std::error::Error;

use autocfg::{emit, AutoCfg};
use rustc_version::{version_meta, Channel};

fn main() -> Result<(), Box<dyn Error + 'static>> {
	let ac = AutoCfg::new()?;
	let version_data = version_meta()?;
	if let Channel::Nightly = version_data.channel {
		if ac.probe_rustc_version(1, 57) {
			emit("docsrs");
		}
	}

	if ac.probe_expression("std::result::Result::<(), ()>::unwrap_unchecked")
		&& ac.probe_expression("std::option::Option::<()>::unwrap_unchecked")
	{
		emit("has_unwrap_unchecked");
	}

	Ok(())
}
