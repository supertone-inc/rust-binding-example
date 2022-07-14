use libc::{c_float, c_int, c_void, size_t};

#[no_mangle]
pub extern "C" fn hello__callback__map(
    in_items: *const c_int,
    out_items: *mut c_float,
    item_count: size_t,
    mapper: extern "C" fn(item: c_int) -> c_float,
) {
    let items: &[c_int] = unsafe { std::slice::from_raw_parts(in_items, item_count) };
    let mapped_items = hello::callback::map(items, |item| mapper(*item));

    unsafe { std::ptr::copy_nonoverlapping(mapped_items.as_ptr(), out_items, item_count) }
}

#[no_mangle]
pub extern "C" fn hello__callback__map_with_state(
    in_items: *const c_int,
    out_items: *mut c_float,
    item_count: size_t,
    state: *const c_void,
    mapper: extern "C" fn(item: c_int, state: *const c_void) -> c_float,
) {
    let items: &[c_int] = unsafe { std::slice::from_raw_parts(in_items, item_count) };
    let mapped_items = hello::callback::map(items, |item| mapper(*item, state));

    unsafe { std::ptr::copy_nonoverlapping(mapped_items.as_ptr(), out_items, item_count) }
}
