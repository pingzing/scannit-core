use libc::c_char;
use std::ffi::CString;

pub fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

pub fn free_buffer<T>(buf: FFIBuffer<T>) {
    let mut vector = unsafe { std::vec::Vec::from_raw_parts(buf.data, buf.len, buf.capacity) };
    let reconstituted_vector = vector.as_mut_ptr();
    unsafe {
        Box::from_raw(reconstituted_vector);
    }
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
