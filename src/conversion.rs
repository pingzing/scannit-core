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

pub fn get_bits_as_u8(bytes: &[u8], bit_offset_index: usize, bit_length: usize) -> u8 {
    get_bits_as_u64(bytes, bit_offset_index, bit_length) as u8
}

pub fn get_bits_as_u16(bytes: &[u8], bit_offset_index: usize, bit_length: usize) -> u16 {
    get_bits_as_u64(bytes, bit_offset_index, bit_length) as u16
}

pub fn get_bits_as_u32(bytes: &[u8], bit_offset_index: usize, bit_length: usize) -> u32 {
    get_bits_as_u64(bytes, bit_offset_index, bit_length) as u32
}

pub fn get_bits_as_u64(bytes: &[u8], bit_offset_index: usize, bit_length: usize) -> u64 {
    let byte_offset = bit_offset_index / 8;
    let end_byte_offset = (bit_length / 8) + byte_offset + 1;
    let num_relevant_bytes = end_byte_offset - byte_offset;
    let num_bits_to_mask = bit_offset_index % 8;
    let num_bits_to_shift = (num_relevant_bytes * 8) - (num_bits_to_mask + bit_length);
    let mut and_mask = 0u64;
    for _ in 0..num_bits_to_mask {
        and_mask = (and_mask << 1) + 1;
    }
    // Shift the 1s to the top of the bit-value, and negate the whole number to generate our mask
    and_mask = !(and_mask << (64 - num_bits_to_mask));
    let bytes = &bytes[byte_offset..end_byte_offset];
    let bit_position = 56;
    // 1) Weld all the bytes together, moving up the number to make space for new bytes as we go
    let mut welded_bytes = 0u64;
    for (i, byte) in bytes.iter().enumerate() {
        welded_bytes += (*byte as u64) << (bit_position - (i * 8))
    }
    // 2) AND off the top bits that aren't part of what we want
    // 3) Shift off the bottom bits that aren't part of what we want
    // 4) And shift rightward based on what the user requested of us
    ((welded_bytes & and_mask) >> num_bits_to_shift) >> ((8 - num_relevant_bytes) * 8)
}

#[cfg(test)]
mod test {
    use crate::conversion::{get_bits_as_u16, get_bits_as_u32, get_bits_as_u64, get_bits_as_u8};

    #[test]
    fn to_bits_should_handle_trailing_and_leading_bits() {
        // want to get the middle 11 bits of a value that is divided like this:
        // -------- ---XXXXX XXXXXX-- where the Xs are the values we want.
        // They begin at index 11, and the value is 11 bits long.
        let three_bytes: [u8; 3] = [0x00, 0x1F, 0xFC];
        let expected = 2047u64;
        let actual = get_bits_as_u64(&three_bytes, 11, 11);
        assert_eq!(expected, actual);
    }

    #[test]
    fn to_bits_should_handle_u8() {
        let byte: [u8; 1] = [0b00110100];
        let expected = 0b00001101;
        // try to pull out of the four bits that span a nybble boundary
        let actual = get_bits_as_u8(&byte, 2, 4);
        assert_eq!(expected, actual);
    }

    #[test]
    fn to_bits_should_handle_u16() {
        // Interested in:          |----------------------| <-- those 16 bits
        let bytes: [u8; 3] = [0b1111_0001, 0b0000_0001, 0b0010_0000];
        let expected = 0b10001000_00001001;
        let actual = get_bits_as_u16(&bytes, 3, 16);
        assert_eq!(expected, actual);
    }

    #[rustfmt::skip]
    #[test]
    fn to_bits_should_handle_u32() {
        // Interested in:               |--------------------------------------------------| <-- these 32 bits
        let bytes: [u8; 5] = [0b1100_1001, 0b0000_1000, 0b0000_0001, 0b1101_0000, 0b0110_0011];
        let expected: u32 = 0b10000100_00000000_11101000_00110001;
        let actual = get_bits_as_u32(&bytes, 7, 32);
        assert_eq!(expected, actual);
    }
}
