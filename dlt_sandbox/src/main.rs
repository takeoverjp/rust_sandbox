use std::ffi::CString;

#[link(name = "dlt")]
extern "C" {
}
// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let mut c_str = CString::new("Hello FFI").unwrap();
        dlt_log_init(DLT_LOG_TO_CONSOLE.try_into().unwrap());
        dlt_log(DltLogLevelType_DLT_LOG_FATAL, c_str.as_ptr().cast_mut());
    }
}
