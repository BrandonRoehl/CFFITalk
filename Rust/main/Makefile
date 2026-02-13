a.out: target/release/rust-main
	ln -sf $(PWD)/$< $@

target/release/rust-main: build.rs src/* ../../link/* ../../link.h
	cargo build --release

