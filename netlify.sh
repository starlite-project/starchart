rustup toolchain install nightly-x86_64-unknown-linux-gnu

export RUSTDOCFLAGS="-D broken_intra_doc_links"

cargo +nightly doc --workspace --no-deps --all-features

echo '<meta http-equiv="refresh" content="0;url=starchart/index.html">' > target/doc/index.html