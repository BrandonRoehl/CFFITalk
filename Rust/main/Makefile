a.out: target/release/rust-main
	ln -sf $(PWD)/$< $@

target/release/rust-main: src/* ../../link/* ../../link.h
	cargo build --release

