use std::cell::RefCell;
use std::ffi::{c_char, c_int};
use std::{ptr, slice};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hello(#[from] hello::error::Error),
}

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<Error>>> = RefCell::new(None);
}

fn update_last_error(err: Error) {
    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

fn take_last_error() -> Option<Box<Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

#[no_mangle]
pub extern "C" fn hello__error__get_last_error_message_length() -> usize {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len(),
        None => 0,
    })
}

#[no_mangle]
pub unsafe extern "C" fn hello__error__get_last_error_message(
    buffer: *mut c_char,
    buffer_length: usize,
) -> usize {
    if buffer.is_null() {
        return 0;
    }

    let error_message = match take_last_error() {
        Some(err) => err.to_string(),
        None => "".to_string(),
    };

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, buffer_length);
    let copy_length = std::cmp::min(buffer_length, error_message.len());

    ptr::copy_nonoverlapping(error_message.as_ptr(), buffer.as_mut_ptr(), copy_length);

    if buffer.len() > error_message.len() {
        buffer[error_message.len()] = b'\0';
    }

    copy_length
}

#[no_mangle]
pub unsafe extern "C" fn hello__error__throw_error() -> c_int {
    match hello::error::throw_error() {
        Ok(_) => 0,
        Err(err) => {
            update_last_error(err.into());
            -1
        }
    }
}
