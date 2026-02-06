.PHONY: all
all: rust c go swift

.PHONY: c
c: C/libc_get_set.a C/libc_next.a C/libc_step.a

.SECONDEXPANSION:
C/libc_%.a: C/$$*.o
	$(AR) rcs $@ $^

.PHONY: swift
swift:
	cd Swift; swift build -c release

.PHONY: go
go: Go/libgo_get_set.a Go/libgo_next.a Go/libgo_step.a

Go/libgo_%.a: Go/*/*.go
	cd go; go build -buildmode=c-archive -o $(@F) ./$*

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


