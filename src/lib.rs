// Copyright (C) 2022 Antti Keränen
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 3.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
mod pregenerated;

#[cfg(not(feature = "bindgen"))]
pub use pregenerated::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_bqn_ffi() {
        let code = CString::new("2+2").unwrap();
        let ret = unsafe {
            bqn_init();
            bqn_toF64(bqn_evalCStr(code.as_ptr()))
        };
        assert_eq!(ret, 4.0);
    }
}
