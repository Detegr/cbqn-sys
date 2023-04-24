// Copyright (C) 2022 Antti Keränen
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 3.

#[cfg(feature = "bindgen")]
use std::{env, path::PathBuf};

#[cfg(feature = "bindgen")]
use bindgen;

fn main() {

    #[cfg(feature = "shared-object")]
    {
        println!("cargo:rustc-link-lib=cbqn");
    };

    #[cfg(feature = "bindgen")]
    {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
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
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");

        println!("cargo:rerun-if-changed=CBQN/include/bqnffi.h");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
