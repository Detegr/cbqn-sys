#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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
