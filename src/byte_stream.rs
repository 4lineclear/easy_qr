use std::ops::Deref;

#[derive(Debug, Default)]
pub struct Bytes {
    bytes: Vec<u8>,
    shift: u32,
}

impl Bytes {
    /// Push a number `data`, according to the given `width`
    ///
    /// This code is ripped off(with some adjustments) from
    /// <https://github.com/kennytm/qrcode-rust/blob/master/src/bits.rs>
    ///
    /// # Panics
    ///
    /// unlikely
    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn push(&mut self, data: u16, width: u16) {
        debug_assert!(
            width == 16 || width < 16 && data < (1 << width),
            "{data} is not a {width} bit number",
        );
        let sum = width as u32 + self.shift;
        match (self.shift, sum) {
            (0, 0..=8) => {
                self.bytes.push((data << (8 - sum)) as u8);
            }
            (0, _) => {
                self.bytes.push((data >> (sum - 8)) as u8);
                self.bytes.push((data << (16 - sum)) as u8);
            }
            (_, 0..=8) => {
                *self.bytes.last_mut().unwrap() |= (data << (8 - sum)) as u8;
            }
            (_, 9..=16) => {
                *self.bytes.last_mut().unwrap() |= (data >> (sum - 8)) as u8;
                self.bytes.push((data << (16 - sum)) as u8);
            }
            _ => {
                *self.bytes.last_mut().unwrap() |= (data >> (sum - 8)) as u8;
                self.bytes.push((data >> (sum - 16)) as u8);
                self.bytes.push((data << (24 - sum)) as u8);
            }
        }
        // mod 8, probably optimized out anyways
        self.shift = sum & 0b111;
    }
    /// Returns a new [Bytes] with a [Vec] of the given capacity
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
            shift: 0,
        }
    }
    /// Disassembles the stream into its bytes and the leftover
    /// bits in the last inserted byte
    #[must_use]
    pub fn into_parts(self) -> (Vec<u8>, u32) {
        let Self { bytes, shift } = self;
        (bytes, shift)
    }
    pub(crate) fn push_full_byte(&mut self, data: u8) {
        self.shift = 0;
        self.bytes.push(data);
    }
    #[must_use]
    pub const fn shift(&self) -> u32 {
        self.shift
    }
}

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
