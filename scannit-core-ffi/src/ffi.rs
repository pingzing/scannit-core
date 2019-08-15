use crate::models::FFITravelCard;
use libc::c_char;
use std::ffi::CString;

// Example of how to return a string to an external C-like caller
#[no_mangle]
extern "C" fn transfer_string() -> *const c_char {
    let example_string = String::from("I am a string from Rust.");
    let c_string = CString::new(example_string).unwrap();

    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

// Example of how to return a vec of strings to an external C-like caller.
#[no_mangle]
extern "C" fn get_vector() -> FFIBuffer<*mut c_char> {
    let strings_vector = vec!["Rust string one".to_string(), "Rust string two".to_string()];
    let mut buffer: Vec<*mut i8> = strings_vector
        .into_iter()
        .map(|x| CString::new(x).unwrap().into_raw())
        .collect();
    let data = buffer.as_mut_ptr();
    let len = buffer.len();
    let capacity = buffer.capacity();
    std::mem::forget(buffer); // Leak the memory so we don't auto-drop it
    FFIBuffer::<*mut c_char> {
        data,
        len,
        capacity,
    }
}

#[no_mangle]
pub extern "C" fn free_string_buffer(buf: FFIBuffer<*mut c_char>) {
    let mut vector = unsafe { std::vec::Vec::from_raw_parts(buf.data, buf.len, buf.capacity) };
    for string in vector.iter() {
        free_string(*string);
    }
    let vector = vector.as_mut_ptr();
    unsafe {
        // Take ownership of the data pointed to by the box,
        Box::from_raw(vector);
        // ...and destroy it at the end of scope.
    }
}

#[no_mangle]
pub extern "C" fn free_byte_buffer(buf: FFIBuffer<u8>) {
    let mut vector = unsafe { std::vec::Vec::from_raw_parts(buf.data, buf.len, buf.capacity) };
    let reconstituted_vector = vector.as_mut_ptr();
    unsafe {
        Box::from_raw(reconstituted_vector);
    }
}

#[no_mangle]
pub extern "C" fn free_travel_card(travel_card: FFITravelCard) {
    // Deallocate: The string
    // and all the FFIBuffers
}

#[repr(C)]
pub struct FFIBuffer<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> From<&mut Vec<T>> for FFIBuffer<T> {
    /// Transform the given vector into an FFI-friendly buffer.
    /// Does NOT call forget() on the underlying vector.
    fn from(val: &mut Vec<T>) -> Self {
        let data = val.as_mut_ptr();
        let len = val.len();
        let capacity = val.capacity();
        FFIBuffer::<T> {
            data,
            len,
            capacity,
        }
    }
}
