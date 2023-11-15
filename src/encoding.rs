use super::{EncodingMode, ErrorCorrection};

use super::QRCodeVersion;

pub const BYTE_WIDTH: usize = 8;
pub const CHAR_WIDTH: usize = 10;

#[cfg(test)]
mod test;

pub trait Encodable {
    fn create_bits(self, version: QRCodeVersion, ec: ErrorCorrection) -> (Vec<u8>, EncodingMode);
}

impl Encodable for &str {
    fn create_bits(self, version: QRCodeVersion, ec: ErrorCorrection) -> (Vec<u8>, EncodingMode) {
        use EncodingMode::*;
        let mode = EncodingMode::analyze_string(self);
        (
            (match mode {
                Numeric => encode_numeric,
                Alphanumeric => encode_alphanumeric,
                Byte => todo!(),
            })(self, version, ec),
            mode,
        )
    }
}

/// Returns the number of bits unused in the last inputted [byte](u8)
#[allow(clippy::cast_possible_truncation)]
pub fn add_start_bits(
    vec: &mut Vec<u8>,
    version: QRCodeVersion,
    mode: EncodingMode,
    count: usize,
) -> usize {
    let count_bits = count_bits_count(version, mode);
    let mode = mode as usize;

    // the combination of `mode` and `count` bits
    let bits = mode << count_bits | count;
    let mut bits_left = 4 + count_bits;
    while bits_left >= BYTE_WIDTH {
        // shifts just so that the needed window of 8 bits is taken
        vec.push((bits >> (bits_left - BYTE_WIDTH)) as u8);
        bits_left -= BYTE_WIDTH;
    }
    // pushes the remaining that couldn't be shoved into a byte,
    // returning the amount unused on the last byte
    vec.push((bits << (BYTE_WIDTH - bits_left)) as u8);
    BYTE_WIDTH - bits_left
}

pub fn encode_alphanumeric(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    todo!()
}

/// Encodes the given numeric digit only [`str`]
/// into the data segment of a QR Code
///
/// Careful, lots of purposeful truncation done below
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn encode_numeric(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    let required_code_words = version.data_size(ec);
    let mut result = Vec::with_capacity(required_code_words);
    let mut unused = add_start_bits(&mut result, version, EncodingMode::Numeric, s.len());
    let chunks = s.as_bytes().chunks_exact(3);
    let remaining_digits = chunks.remainder();

    for bits in chunks.map(group_3) {
        match unused {
            // can simply allocate two new bytes, as the previous byte is filled
            0 => {
                result.push((bits >> 2) as u8);
                result.push((bits << 6) as u8);
                unused = 6;
            }
            // in the (impossible) case that one bit is unused
            // this case is special as `unused` will remain as 1 and
            // will always require 3 bytes to be changed (1 mutated, 2 added)
            1 => {
                *result.last_mut().unwrap() |= (bits >> 7) as u8;
                result.push((bits >> 1) as u8);
                result.push((bits << 7) as u8);
            }
            // for any other case
            _ => {
                // `CHAR_WIDTH - unused`: shift to align to the rightmost unused bits
                *result.last_mut().unwrap() |= (bits >> (CHAR_WIDTH - unused)) as u8;
                // `unused - 2`: shift push remaining bits to the left
                // of the new byte accounting for the fact that the given
                // number is 2 bits longer than a u8
                result.push((bits << (unused - 2)) as u8);
                unused -= 2;
            }
        }
    }
    // encode the remaining digits
    match remaining_digits {
        [] => (),
        [b] => {
            /// 4 is used since a single digit takes up to 4 bits to represent
            const DIGIT_WIDTH: usize = 4;
            let digit = b - b'0';
            match unused {
                0 => {
                    result.push(digit << DIGIT_WIDTH);
                    unused = DIGIT_WIDTH;
                }
                1..=3 => {
                    *result.last_mut().unwrap() |= digit >> (DIGIT_WIDTH - unused);
                    result.push(digit << (DIGIT_WIDTH + unused));
                    unused += DIGIT_WIDTH;
                }
                _ => {
                    *result.last_mut().unwrap() |= digit << (BYTE_WIDTH - unused);
                    unused -= DIGIT_WIDTH;
                }
            }
        }
        [b1, b2] => {
            /// 7 is used since two digit takes up to 7 bits to represent
            const DIGIT_WIDTH: usize = 7;
            let digit = (b1 - b'0') * 10 + (b2 - b'0');
            match dbg!(unused) {
                0 => {
                    result.push(digit << (BYTE_WIDTH - DIGIT_WIDTH));
                    unused = 1;
                }
                DIGIT_WIDTH => {
                    *result.last_mut().unwrap() |= digit;
                    unused = 0;
                }
                _ => {
                    *result.last_mut().unwrap() |= digit >> (DIGIT_WIDTH - unused);
                    result.push(digit << (unused + 1));
                }
            }
        }
        _ => unreachable!(),
    }
    // making sure there are enough termination bits (4)
    // if end of length is reached, then it is unnecessary
    if unused < 4 && result.len() < required_code_words {
        result.push(0x00);
    }
    let mut filler = [0xec, 0x11].into_iter().cycle();
    while result.len() < required_code_words {
        result.push(filler.next().unwrap());
    }
    result
}

/// Takes the given slice of bytes, returning the sum of the integers
/// held within `0..3`
///
/// Assumes the given slice contains only the bytes `b'0'..=b'9'`
///
/// # Safety
///
/// Assumes that the given bytes are made up only of decimal digits
#[inline]
pub(crate) const fn group_3(bytes: &[u8]) -> u16 {
    (bytes[0] - b'0') as u16 * 100 + (bytes[1] - b'0') as u16 * 10 + (bytes[2] - b'0') as u16
}

#[inline]
#[must_use]
pub const fn count_bits_count(version: QRCodeVersion, encoding: EncodingMode) -> usize {
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
