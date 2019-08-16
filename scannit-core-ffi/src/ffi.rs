use crate::models::FFIHistory;
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

pub fn free_byte_buffer(buf: FFIByteBuffer) {
    unsafe { std::vec::Vec::from_raw_parts(buf.data, buf.len, buf.capacity) };
}

pub fn free_history_buffer(buf: FFIHistoryBuffer) {
    unsafe { std::vec::Vec::from_raw_parts(buf.data, buf.len, buf.capacity) };
}

#[repr(C)]
pub struct FFIByteBuffer {
    data: *mut u8,
    len: usize,
    capacity: usize,
}

impl From<&mut Vec<u8>> for FFIByteBuffer {
    /// Transform the given vector into an FFI-friendly buffer.
    /// Does NOT call forget() on the underlying vector.
    fn from(val: &mut Vec<u8>) -> Self {
        let data = val.as_mut_ptr();
        let len = val.len();
        let capacity = val.capacity();
        FFIByteBuffer {
            data,
            len,
            capacity,
        }
    }
}

#[repr(C)]
pub struct FFIHistoryBuffer {
    data: *mut FFIHistory,
    len: usize,
    capacity: usize,
}

impl From<&mut Vec<FFIHistory>> for FFIHistoryBuffer {
    fn from(val: &mut Vec<FFIHistory>) -> Self {
        let data = val.as_mut_ptr();
        let len = val.len();
        let capacity = val.capacity();
        FFIHistoryBuffer {
            data,
            len,
            capacity,
        }
    }
}
