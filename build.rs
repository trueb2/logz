extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=include/bridge.h");

    let llvm_config_path = Command::new("llvm-config")
        .args(["--prefix"])
        .output()
        .expect("`llvm-config` must be in PATH")
        .stdout;
    let llvm_config_path =
        String::from_utf8(llvm_config_path).expect("`llvm-config --prefix` output must be UTF-8");

    std::env::set_var(
        "LLVM_CONFIG_PATH",
        format!("{}/bin/llvm-config", llvm_config_path.trim()),
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/bridge.h")
        .allowlist_function("logz_rs_error_handler")
        .allowlist_function("log_.*")
        .allowlist_var("CONFIG_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Tell cargo that bindings will be no_std
        .use_core()
        .ctypes_prefix("cty")
        // Don't create layout tests that would only run on desktop
        .layout_tests(false)
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
