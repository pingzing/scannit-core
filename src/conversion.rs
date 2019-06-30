use std::fmt::Write;

///Converts an array of bytes into a hex string.
pub fn as_hex_string(bytes: &[u8]) -> String {
    let mut string_buffer = String::new();
    for &byte in bytes {
        // Writing to a String never fails, so ignore the Result.
        let _ = write!(&mut string_buffer, "{:x}", byte);
    }
    string_buffer
}
