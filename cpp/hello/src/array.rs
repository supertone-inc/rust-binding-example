use libc::{c_float, size_t};

#[no_mangle]
pub extern "C" fn hello__array__concat(
    a: *const c_float,
    a_length: size_t,
    b: *const c_float,
    b_length: size_t,
) -> *mut c_float {
    let a: &[f32] = unsafe { std::slice::from_raw_parts(a, a_length) };
    let b: &[f32] = unsafe { std::slice::from_raw_parts(b, b_length) };
    let c = hello::array::concat(a, b).into_boxed_slice();
    Box::into_raw(c) as *mut c_float
}

#[no_mangle]
pub unsafe extern "C" fn hello__array__destroy_array(array: *mut c_float) {
    let _ = Box::from_raw(array);
}

#[no_mangle]
pub extern "C" fn hello__array__concat_safe(
    a: *const c_float,
    a_length: size_t,
    b: *const c_float,
    b_length: size_t,
    out_array: *mut c_float,
) {
    let a: &[f32] = unsafe { std::slice::from_raw_parts(a, a_length) };
    let b: &[f32] = unsafe { std::slice::from_raw_parts(b, b_length) };
    let c = hello::array::concat(a, b);

    unsafe { std::ptr::copy_nonoverlapping(c.as_ptr(), out_array, c.len()) }
}
