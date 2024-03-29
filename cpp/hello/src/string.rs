use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn hello__string__to_uppercase_alloc(string: *const c_char) -> *mut c_char {
    let string = CStr::from_ptr(string).to_str().unwrap();
    let string = hello::string::to_uppercase(string);
    CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn hello__string__destroy_string(string: *mut c_char) {
    let _ = CString::from_raw(string);
}

#[no_mangle]
pub unsafe extern "C" fn hello__string__to_uppercase(
    in_string: *const c_char,
    out_string: *mut c_char,
) {
    let in_string = CStr::from_ptr(in_string).to_str().unwrap();
    let string = hello::string::to_uppercase(in_string);
    let string = CString::new(string).unwrap();
    let bytes = string.as_bytes();

    std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_string as *mut u8, bytes.len())
}
