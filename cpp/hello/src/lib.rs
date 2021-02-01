use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
pub extern fn print_string(name: *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };
    hello::print_string(c_str.to_str().unwrap());
}

#[no_mangle]
pub extern fn get_string(name: *const c_char) -> *mut c_char {
    let c_name: &CStr = unsafe { CStr::from_ptr(name) };
    let string = hello::get_string(c_name.to_str().unwrap());
    CString::new(string).unwrap().into_raw()
}
