#[macro_use]
extern crate error_chain;

mod errors;
use errors::*;

use libc::{c_char, c_float, c_int, size_t};
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn to_uppercase(string: *const c_char) -> *mut c_char {
    let string = unsafe { CStr::from_ptr(string) }.to_str().unwrap();
    let string = hello::to_uppercase(string);
    CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn destroy_string(string: *mut c_char) {
    let _ = CString::from_raw(string);
}

#[no_mangle]
pub extern "C" fn to_uppercase_safe(in_string: *const c_char, out_string: *mut c_char) {
    let in_string = unsafe { CStr::from_ptr(in_string) }.to_str().unwrap();
    let string = hello::to_uppercase(in_string);
    println!("RUSTSTRING: {}", string.len());
    let string = CString::new(string).unwrap();

    let bytes = string.as_bytes();
    println!("CSTRING: {}", bytes.len());

    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_string as *mut u8, bytes.len()) }
}

#[no_mangle]
pub extern "C" fn concat(
    a: *const c_float,
    a_length: size_t,
    b: *const c_float,
    b_length: size_t,
) -> *mut c_float {
    let a: &[f32] = unsafe { std::slice::from_raw_parts(a, a_length) };
    let b: &[f32] = unsafe { std::slice::from_raw_parts(b, b_length) };
    let c = hello::concat(a, b).into_boxed_slice();
    Box::into_raw(c) as *mut c_float
}

#[no_mangle]
pub unsafe extern "C" fn destroy_array(array: *mut c_float) {
    let _ = Box::from_raw(array);
}

#[no_mangle]
pub extern "C" fn concat_safe(
    a: *const c_float,
    a_length: size_t,
    b: *const c_float,
    b_length: size_t,
    out_array: *mut c_float,
) {
    let a: &[f32] = unsafe { std::slice::from_raw_parts(a, a_length) };
    let b: &[f32] = unsafe { std::slice::from_raw_parts(b, b_length) };
    let c = hello::concat(a, b);

    unsafe { std::ptr::copy_nonoverlapping(c.as_ptr(), out_array, c.len()) }
}

#[no_mangle]
pub unsafe extern "C" fn raise_error() -> c_int {
    match hello::raise_error() {
        Ok(_) => return 0,
        Err(err) => {
            update_last_error(err.into());
            return -1;
        }
    };
}
