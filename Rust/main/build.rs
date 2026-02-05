use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut base_dir = PathBuf::from(manifest_dir);
    base_dir.pop();
    base_dir.pop();
    let c_dir = base_dir.join("C");

    for dir in ["libc_get_set.a", "libc_next.a", "libc_step.a"] {
        let dir_path = c_dir.join(dir);
        // println!("cargo:rustc-link-lib=static={}", dir_path.display());
        println!("cargo:rustc-link-lib=static={}", dir_path.display());
    }
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(base_dir.join("link.h").to_str().unwrap())
        // No implicit copy
        .derive_copy(false)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
