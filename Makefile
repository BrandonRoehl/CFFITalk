.PHONY: all
all: rust c

.PHONY: c
c: C/libc_get_set.a C/libc_next.a C/libc_step.a

.SECONDEXPANSION:
C/libc_%.a: C/$$*.o
	$(AR) rcs $@ $^

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


