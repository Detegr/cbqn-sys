use bindgen;
use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("make")
        .arg("shared-o3")
        .arg(format!("OUTPUT={}/libcbqn.so", out_dir))
        .current_dir("CBQN")
        .spawn()
        .expect("make");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("CBQN/include/bqnffi.h")
        .allowlist_function("bqn_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(PathBuf::from(&out_dir[..]).join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=cbqn");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=CBQN/include/bqnffi.h");
}
