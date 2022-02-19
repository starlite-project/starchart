// this only exists so that OUT_DIR is set throughout all the tests

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
}
