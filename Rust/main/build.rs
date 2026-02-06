use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell Cargo where to find the library file (e.g., in the current directory)
    // 'native' kind is generally used for system or precompiled libraries
    println!("cargo:rustc-link-search=../../link/");

    // Link Xcode if its found
    if let Ok(output) = Command::new("xcode-select").arg("-p").output() {
        if output.status.success() {
            let xcode_path = String::from_utf8_lossy(&output.stdout);
            println!(
                "cargo:rustc-link-search={}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx",
                xcode_path
            );
        }
    }
    // $(CC) C/main.c -L link -L "$(shell xcode-select -p)" \

    for f in ["c_get_set", "c_next", "c_step"] {
        println!("cargo:rustc-link-lib=static={}", f);
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../../link.h")
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
// let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
// let mut base_dir = PathBuf::from(manifest_dir);
// base_dir.pop();
// base_dir.pop();
// let c_dir = base_dir.join("C");

// The bindgen::Builder is the main entry point
// to bindgen, and lets you build up options for
// the resulting bindings.
