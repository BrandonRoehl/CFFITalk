
.PHONY: all
all: rust

.PHONY: rust
rust: \
	Rust/get_set/target/release/librust_life_get_set.a \
	Rust/next/target/release/librust_life_next.a \
	Rust/step/target/release/librust_life_step.a

Rust/get_set/target/release/librust_life_get_set.a: Rust/get_set/src/*.rs
	cd Rust/get_set && cargo build --release

Rust/next/target/release/librust_life_next.a: Rust/next/src/*.rs
	cd Rust/next && cargo build --release

Rust/step/target/release/librust_life_step.a: Rust/step/src/*.rs
	cd Rust/step && cargo build --release


