
RUST_LIBS = \
			Rust/get_set/target/debug/librust_life_get_set.a \
			Rust/next/target/debug/librust_life_next.a \
			Rust/step/target/debug/librust_life_step.a

$(RUST_LIBS): Rust/%: Rust/*/src/*.rs
	@echo "* $*"
	@echo "^ $^"
	@echo "@D $(@D)"
	@echo "% $%"

.PHONY: rust
rust: $(RUST_LIBS)
