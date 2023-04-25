use std::ffi::c_float;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn hello__array__concat_alloc(
    a: *const c_float,
    a_length: usize,
    b: *const c_float,
    b_length: usize,
) -> *mut c_float {
    let a: &[f32] = std::slice::from_raw_parts(a, a_length);
    let b: &[f32] = std::slice::from_raw_parts(b, b_length);
    let c = hello::array::concat(a, b).into_boxed_slice();
    Box::into_raw(c) as *mut c_float
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn hello__array__destroy_array(array: *mut c_float) {
    let _ = Box::from_raw(array);
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn hello__array__concat(
    a: *const c_float,
    a_length: usize,
    b: *const c_float,
    b_length: usize,
    out_array: *mut c_float,
) {
    let a: &[f32] = std::slice::from_raw_parts(a, a_length);
    let b: &[f32] = std::slice::from_raw_parts(b, b_length);
    let c = hello::array::concat(a, b);

    std::ptr::copy_nonoverlapping(c.as_ptr(), out_array, c.len())
}
