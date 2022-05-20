use libc::{c_int, size_t};

#[no_mangle]
pub extern "C" fn hello__callback__map_safe(
    items_in: *const c_int,
    items_out: *mut c_int,
    item_count: size_t,
    mapper: extern "C" fn(c_int) -> c_int,
) {
    let items: &[c_int] = unsafe { std::slice::from_raw_parts(items_in, item_count) };
    let mapped_items = hello::callback::map(items, |item| mapper(*item));

    unsafe { std::ptr::copy_nonoverlapping(mapped_items.as_ptr(), items_out, mapped_items.len()) }
}
