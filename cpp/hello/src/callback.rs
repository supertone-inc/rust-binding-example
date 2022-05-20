use libc::{c_float, size_t};

#[no_mangle]
pub extern "C" fn hello__callback__map_safe(
    items_in: *const c_float,
    items_out: *mut c_float,
    item_count: size_t,
    mapper: extern "C" fn(c_float) -> c_float,
) {
    let items: &[c_float] = unsafe { std::slice::from_raw_parts(items_in, item_count) };
    let mapped_items = hello::callback::map(items, |item| mapper(*item));

    unsafe { std::ptr::copy_nonoverlapping(mapped_items.as_ptr(), items_out, mapped_items.len()) }
}
