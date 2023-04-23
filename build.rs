// Copyright (C) 2022 Antti Keränen
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 3.

use std::{env, path::PathBuf};

#[cfg(feature = "bindgen")]
use bindgen;

#[cfg(feature = "shared-object")]
use std::{fs, process::Command};

#[cfg(feature = "shared-object")]
use fs_extra::dir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    #[cfg(feature = "shared-object")]
    {
        let cbqn_dir = out_dir.join("CBQN");
        let bytecode_dir = cbqn_dir.join("build/bytecodeLocal");

        dir::remove(&cbqn_dir).unwrap();

        // build.rs must not modify anything outside $OUT_DIR, so copying CBQN to there
        dir::copy("CBQN", &out_dir, &dir::CopyOptions::new())
            .expect("expected to copy CBQN to $OUT_DIR/CBQN");

        fs::create_dir(&bytecode_dir).expect("expected to create build/bytecodeLocal");
        dir::copy(
            cbqn_dir.join("build/bytecodeSubmodule/gen"),
            &bytecode_dir,
            &dir::CopyOptions::new(),
        )
        .expect(
            "expected to copy prebuilt bytecode to local bytecode to avoid using git during build",
        );

        let clean_first = Command::new("make")
            .arg("clean")
            .current_dir(&cbqn_dir)
            .status()
            .expect("make clean");

        if !clean_first.success() {
            std::process::exit(1);
        }

        let for_build = Command::new("make")
            .arg("for-build")
            .current_dir(&cbqn_dir)
            .status()
            .expect("make for-build");

        if !for_build.success() {
            std::process::exit(1);
        }

        let cbqn_build = Command::new("make")
            .arg("shared-o3")
            .arg("libcbqn.so")
            .current_dir(&cbqn_dir)
            .status()
            .expect("make");

        if !cbqn_build.success() {
            std::process::exit(1);
        }

        println!("cargo:rustc-link-search=native={}", cbqn_dir.display());
        println!("cargo:rustc-link-lib=cbqn");
    };

    #[cfg(feature = "bindgen")]
    {
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
