use libc::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn get_string() -> *const c_char {
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

#[no_mangle]
pub extern "C" fn get_vector() -> Buffer {
    let strings_vector = vec!["Rust string one".to_string(), "Rust string two".to_string()];
    let mut buffer: Vec<*mut c_char> = strings_vector
        .into_iter()
        .map(|x| CString::new(x).unwrap().into_raw())
        .collect();
    let data = buffer.as_mut_ptr();
    let len = buffer.len();
    std::mem::forget(buffer); // Leak the memory so we don't auto-drop it
    Buffer { data, len }
}

#[no_mangle]
pub extern "C" fn free_vector(buf: Buffer) {
    let vector = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
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

#[repr(C)]
pub struct Buffer {
    data: *mut *mut c_char,    
    len: usize,
}
