use std::ffi::{c_int, c_void};

type Counter = hello::structure::Counter;

#[no_mangle]
pub extern "C" fn hello__structure__counter__new(count: c_int) -> *mut c_void {
    Box::into_raw(Box::new(Counter::new(count))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn hello__structure__counter__delete(counter: *mut c_void) {
    let counter = counter as *mut Counter;

    if !counter.is_null() {
        drop(Box::from_raw(counter));
    }
}

#[no_mangle]
pub unsafe extern "C" fn hello__structure__counter__increase(
    counter: *mut c_void,
    amount: c_int,
) -> c_int {
    let counter = &mut *(counter as *mut Counter);
    counter.increase(amount)
}
