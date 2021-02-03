use std::ffi::{CStr, CString};
use libc::{c_char, c_float, size_t};

#[no_mangle]
pub extern fn to_uppercase(s: *const c_char) -> *mut c_char {
    let s = unsafe { CStr::from_ptr(s) }.to_str().unwrap();
    let string = hello::to_uppercase(s);
    CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern fn destroy_string(string: *mut c_char) {
    let _ = CString::from_raw(string);
}

#[no_mangle]
pub extern fn concat(a: *const c_float, a_length: size_t, b: *const c_float, b_length: size_t) -> *mut c_float {
    let a: &[f32] = unsafe { std::slice::from_raw_parts(a, a_length) };
    let b: &[f32] = unsafe { std::slice::from_raw_parts(b, b_length) };
    let c = hello::concat(a, b).into_boxed_slice();
    Box::into_raw(c) as *mut c_float
}

#[no_mangle]
pub unsafe extern fn destroy_array(array: *mut c_float) {
    let _ = Box::from_raw(array);
}
