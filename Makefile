.PHONY: link_all
link_all: \
	link/libc_get_set.a \
	link/libc_next.a \
	link/libc_step.a \
	link/librust_get_set.a \
	link/librust_next.a \
	link/librust_step.a \
	link/libgo_get_set.a \
	link/libgo_next.a \
	link/libgo_step.a \
	link_swift

.PHONY:
link_swift: swift link
	find "$(PWD)/Swift/.build/release/" -type f -name "lib*.a" -exec ln -sf {} link/ \;
	find "$(shell xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx" -type f -name "lib*.a" -exec ln -sf {} link/ \;


.SECONDEXPANSION:
link/librust_%.a: Rust/$$*/target/release/librust_life_$$*.a link
	ln -sf $(PWD)/$< $@

link/libc_%.a: C/libc_%.a link
	ln -sf $(PWD)/$< $@

link/libgo_%.a: Go/libgo_%.a link
	ln -sf $(PWD)/$< $@

link:
	mkdir -p $@

.PHONY: all
all: rust c go swift

.PHONY: c
c: C/libc_get_set.a C/libc_next.a C/libc_step.a

C/libc_%.a: C/%.o
	$(AR) rcs $@ $^

.PHONY: swift
swift:
	cd Swift; swift build -c release

.PHONY: go
go: Go/libgo_get_set.a Go/libgo_next.a Go/libgo_step.a

.SECONDEXPANSION:
Go/libgo_%.a: Go/$$*/*.go
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


