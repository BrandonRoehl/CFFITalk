# CFFITalk

Building all the `.a` files and constructing the `./link/` folder
```bash
make link
```

For building the individual mains and changing their links use their own builders
or example **Makefile** in each directory
- Rust: `cargo build --release`
- Go: `go build ./cli`
- Swift: `swift build -c release`
- C: `main.c -L ../link -l c_get_set -l c_next -l c_step -o a.out`
