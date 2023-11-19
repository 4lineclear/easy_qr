#![allow(clippy::cast_lossless)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::cast_possible_truncation)]
use crate::byte_stream::Bytes;

use super::{EncodingMode, ErrorCorrection, QRCodeVersion};

#[cfg(test)]
mod test;

pub trait Encodable {
    /// Creates the primitive data bits needed to create a QR Code
    fn create_bits(&self, version: QRCodeVersion, ec: ErrorCorrection) -> (Vec<u8>, EncodingMode);
}
impl Encodable for str {
    fn create_bits(&self, version: QRCodeVersion, ec: ErrorCorrection) -> (Vec<u8>, EncodingMode) {
        use EncodingMode::*;
        let mode = EncodingMode::analyze_string(self);
        (
            (match mode {
                Numeric => encode_numeric,
                Alphanumeric => encode_alphanumeric,
                Byte => encode_byte,
            })(self, version, ec),
            mode,
        )
    }
}
#[must_use]
pub fn encode_byte(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    let required_code_words = version.data_size(ec);
    let mut result = Bytes::with_capacity(required_code_words);

    add_start_bits(&mut result, version, EncodingMode::Byte, s.len());
    s.bytes().for_each(|byte| result.push(byte as u16, 8));
    add_final_bits(&mut result, required_code_words);

    result.into_parts().0
}
#[must_use]
pub fn encode_alphanumeric(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    let required_code_words = version.data_size(ec);
    let mut result = Bytes::with_capacity(required_code_words);

    add_start_bits(&mut result, version, EncodingMode::Alphanumeric, s.len());
    s.as_bytes()
        .chunks(2)
        .map(|bytes| {
            bytes
                .iter()
                .map(|&b| byte_to_alphanumeric(b) as u16)
                .fold((0, 1), |(acc, w), b| (acc * 45 + b, w + 5))
        })
        .for_each(|(bits, w)| result.push(bits, w));
    add_final_bits(&mut result, required_code_words);

    result.into_parts().0
}
#[must_use]
pub fn encode_numeric(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    let required_code_words = version.data_size(ec);
    let mut result = Bytes::with_capacity(required_code_words);

    add_start_bits(&mut result, version, EncodingMode::Numeric, s.len());
    s.as_bytes()
        .chunks(3)
        .map(|bytes| {
            bytes
                .iter()
                .map(|b| (b - b'0') as u16)
                .fold((0, 1), |(acc, w), b| (acc * 10 + b, w + 3))
        })
        .for_each(|(bits, w)| result.push(bits, w));
    add_final_bits(&mut result, required_code_words);

    result.into_parts().0
}
/// Returns the number of bits unused in the last inputted [byte](u8)
#[inline]
fn add_start_bits(bytes: &mut Bytes, version: QRCodeVersion, mode: EncodingMode, count: usize) {
    let count_bits = count_bits_count(version, mode);
    let mode = mode as u16;
    bytes.push(mode, 4);
    bytes.push(count as u16, count_bits);
}
/// Adds the final bits including the terminator and filler bits
#[inline]
fn add_final_bits(bytes: &mut Bytes, required_code_words: usize) {
    dbg!(bytes.shift());
    if (bytes.shift() > 4 || bytes.shift() == 0) && bytes.len() < required_code_words {
        bytes.push_full_byte(0);
    }
    let mut i = 0;
    while bytes.len() < required_code_words {
        bytes.push_full_byte([0xec, 0x11][i & 1]);
        i += 1;
    }
}
/// Returns the number of bits to use to represent the count
#[inline]
#[must_use]
pub const fn count_bits_count(version: QRCodeVersion, encoding: EncodingMode) -> u16 {
    use EncodingMode::*;
    use QRCodeVersion::*;
    match version {
        V1 | V2 | V3 | V4 | V5 | V6 | V7 | V8 | V9 => match encoding {
            Numeric => 10,
            Alphanumeric => 9,
            Byte => 8,
        },
        V10 | V11 | V12 | V13 | V14 | V15 | V16 | V17 | V18 | V19 | V20 | V21 | V22 | V23 | V24
        | V25 | V26 => match encoding {
            Numeric => 12,
            Alphanumeric => 11,
            Byte => 16,
        },
        V27 | V28 | V29 | V30 | V31 | V32 | V33 | V34 | V35 | V36 | V37 | V38 | V39 | V40 => {
            match encoding {
                Numeric => 14,
                Alphanumeric => 13,
                Byte => 16,
            }
        }
    }
}
/// Takes the given byte, turning it into its alphanumeric counterpart
///
/// Assumes that the inputted byte is convertible.
/// In the case that the given byte is invalid, a `0` is returned
#[inline]
#[must_use]
pub const fn byte_to_alphanumeric(b: u8) -> u8 {
    match b {
        // 0..=9
        b'0'..=b'9' => b - b'0',
        // 10..=35
        b'A'..=b'Z' => b - b'A' + 10,
        // 36..=44
        b' ' => 36,
        b'$' => 37,
        b'%' => 38,
        b'*' => 39,
        b'+' => 40,
        b'-' => 41,
        b'.' => 42,
        b'/' => 43,
        b':' => 44,
        // Error
        _ => 0,
    }
}
