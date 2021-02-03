use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn to_uppercase(s: *const c_char) -> *mut c_char {
    let string = hello::to_uppercase(unsafe { CStr::from_ptr(s) }.to_str().unwrap());
    CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern fn destroy_string(string: *mut c_char) {
    let _ = CString::from_raw(string);
}
