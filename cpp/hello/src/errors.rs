use libc::{c_char, c_int, size_t};
use std::{cell::RefCell, ptr, slice};

error_chain! {
    links {
        Hello(hello::errors::Error, hello::errors::ErrorKind);
    }
}

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<Error>>> = RefCell::new(None);
}

pub fn update_last_error(err: Error) {
    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

pub fn take_last_error() -> Option<Box<Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

#[no_mangle]
pub extern "C" fn last_error_length() -> size_t {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as size_t,
        None => 0,
    })
}

#[no_mangle]
pub unsafe extern "C" fn last_error_message(buffer: *mut c_char, length: size_t) -> c_int {
    if buffer.is_null() {
        return -1;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => return 0,
    };

    let error_message = last_error.to_string();

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        return -1;
    }

    ptr::copy_nonoverlapping(
        error_message.as_ptr(),
        buffer.as_mut_ptr(),
        error_message.len(),
    );

    buffer[error_message.len()] = 0;

    error_message.len() as c_int
}
