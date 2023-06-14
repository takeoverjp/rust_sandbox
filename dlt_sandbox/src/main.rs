use std::ffi::CString;

mod dlt{
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    
}

fn main() {
    unsafe {
        let c_str = CString::new("Hello FFI").unwrap();
        dlt::dlt_log_init(dlt::DLT_LOG_TO_CONSOLE.try_into().unwrap());
        dlt::dlt_log(dlt::DltLogLevelType_DLT_LOG_FATAL, c_str.as_ptr().cast_mut());
    }
}
