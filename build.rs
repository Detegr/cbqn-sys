// Copyright (C) 2022 Antti Keränen
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 3.

fn main() {
    #[cfg(feature = "shared-object")]
    {
        println!("cargo:rustc-link-lib=cbqn");
    };

    println!("cargo:rerun-if-changed=src/pregenerated.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
