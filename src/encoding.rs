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
                Byte => encode_byte,
            })(self, version, ec),
            mode,
        )
    }
}
#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn encode_byte(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    let required_code_words = version.data_size(ec);
    let mut result = Vec::with_capacity(required_code_words);
    let mut unused = add_start_bits(&mut result, version, EncodingMode::Byte, s.len());
    for byte in s.as_bytes() {
        *result.last_mut().unwrap() |= byte >> (BYTE_WIDTH - unused);
        result.push(byte << unused);
        unused = BYTE_WIDTH - unused;
    }
    add_final_bits(&mut result, required_code_words, unused);
    result
}
#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn encode_alphanumeric(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    #[inline]
    const fn group_2(bytes: &[u8]) -> u16 {
        match *bytes {
            [b] => byte_to_alphanumeric(b) as u16,
            [b1, b2] => byte_to_alphanumeric(b1) as u16 * 45 + byte_to_alphanumeric(b2) as u16,
            _ => unreachable!(),
        }
    }
    /// `AlphaNumeric` codeword width
    const AN_WIDTH: usize = 11;

    let required_code_words = version.data_size(ec);
    let mut result = Vec::with_capacity(required_code_words);
    let mut unused = add_start_bits(&mut result, version, EncodingMode::Alphanumeric, s.len());
    let chunks = s.as_bytes().chunks_exact(2);
    let remaining = chunks.remainder();

    for bits in chunks.map(group_2) {
        const DIFF: usize = AN_WIDTH - BYTE_WIDTH;
        match unused {
            0..=2 => {
                *result.last_mut().unwrap() |= (bits >> (AN_WIDTH - unused)) as u8;
                result.push((bits >> (DIFF - unused)) as u8);
                result.push((bits << (BYTE_WIDTH - DIFF + unused)) as u8);
                unused = BYTE_WIDTH - DIFF + unused;
            }
            _ => {
                *result.last_mut().unwrap() |= (bits >> (AN_WIDTH - unused)) as u8;
                result.push((bits << (unused - DIFF)) as u8);
                unused -= DIFF;
            }
        }
    }
    if !remaining.is_empty() {
        let remaining = byte_to_alphanumeric(remaining[0]);
        println!("{remaining:#010b} {unused}");
        const REMAINDER_WIDTH: usize = 6;
        const DIFF: usize = BYTE_WIDTH - REMAINDER_WIDTH;
        match unused {
            ..=4 => {
                *result.last_mut().unwrap() |= remaining >> (REMAINDER_WIDTH - unused);
                result.push(remaining << (DIFF + unused));
                unused = DIFF + unused;
            }
            _ => {
                *result.last_mut().unwrap() |= remaining << (unused - REMAINDER_WIDTH);
                unused -= REMAINDER_WIDTH;
            }
        }
    }
    add_final_bits(&mut result, required_code_words, unused);
    result
}

/// Encodes the given numeric digit only [`str`]
/// into the data segment of a QR Code
///
/// Careful, lots of purposeful truncation done below
#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn encode_numeric(s: &str, version: QRCodeVersion, ec: ErrorCorrection) -> Vec<u8> {
    #[inline]
    const fn group_3(bytes: &[u8]) -> u16 {
        (bytes[0] - b'0') as u16 * 100 + (bytes[1] - b'0') as u16 * 10 + (bytes[2] - b'0') as u16
    }
    let required_code_words = version.data_size(ec);
    let mut result = Vec::with_capacity(required_code_words);
    let mut unused = add_start_bits(&mut result, version, EncodingMode::Numeric, s.len());
    let chunks = s.as_bytes().chunks_exact(3);
    let remaining_digits = chunks.remainder();

    for bits in chunks.map(group_3) {
        const DIFF: usize = CHAR_WIDTH - BYTE_WIDTH;
        match unused {
            // can simply allocate two new bytes, as the previous byte is filled
            0 => {
                result.push((bits >> DIFF) as u8);
                result.push((bits << (BYTE_WIDTH - DIFF)) as u8);
                unused = BYTE_WIDTH - DIFF;
            }
            // in the (unlikely) case that one bit is unused
            // this case is special as `unused` will remain as 1 and
            // will always require 3 bytes to be changed (1 mutated, 2 added)
            1 => {
                *result.last_mut().unwrap() |= (bits >> 7) as u8;
                result.push((bits >> 1) as u8);
                result.push((bits << 7) as u8);
            }
            // for any other case
            _ => {
                *result.last_mut().unwrap() |= (bits >> (CHAR_WIDTH - unused)) as u8;
                result.push((bits << (unused - DIFF)) as u8);
                unused -= DIFF;
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
            match unused {
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
                    unused += 1;
                }
            }
        }
        _ => unreachable!(),
    }
    add_final_bits(&mut result, required_code_words, unused);
    result
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

/// Adds filler bits until desired length is achieved
#[inline]
#[allow(clippy::missing_panics_doc)]
pub fn add_final_bits(result: &mut Vec<u8>, required_code_words: usize, unused: usize) {
    // making sure there are enough termination bits (4)
    // if end of length is reached, then it is unnecessary
    if unused < 4 && result.len() < required_code_words {
        result.push(0x00);
    }
    let mut filler = [0xec, 0x11].into_iter().cycle();
    while result.len() < required_code_words {
        result.push(filler.next().unwrap());
    }
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
