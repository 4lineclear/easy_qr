#![deny(
    clippy::all,
    clippy::pedantic,
    // clippy::cargo,
    clippy::nursery,
    // missing_docs,
    rustdoc::all,
    future_incompatible
)]
#![warn(missing_debug_implementations)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

pub mod encoding;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCorrection {
    L,
    M,
    Q,
    H,
}
/// Denotes the alphabet used to encode
///
/// Kanji currently not added.
#[derive(Debug, Clone, Copy)]
pub enum EncodingMode {
    Numeric = 0b0001,
    Alphanumeric = 0b0010,
    Byte = 0b0100,
}

impl EncodingMode {
    #[inline]
    #[must_use]
    pub const fn analyze_string(s: &str) -> Self {
        use EncodingMode::*;
        let mut mode = Numeric;
        let mut i = 0;
        while i < s.as_bytes().len() {
            match s.as_bytes()[i] {
                b'0'..=b'9' => (),
                b'A'..=b'Z' | b' ' | b'$' | b'%' | b'*' | b'+' | b'-' | b'.' | b'/' | b':' => {
                    mode = Alphanumeric;
                }
                _ => return Byte,
            }
            i += 1;
        }
        mode
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QRCodeVersion {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
    V32,
    V33,
    V34,
    V35,
    V36,
    V37,
    V38,
    V39,
    V40,
}

macro_rules! impl_qr_version {
    (
        $(
            (
                $version:ident,
                $l      :literal,
                $m      :literal,
                $q      :literal,
                $h      :literal
            )
        ),*
    ) => {
        impl QRCodeVersion {
            #[inline]
            #[must_use]
            pub const fn fit_version(count: usize, ec: ErrorCorrection) -> Option<Self> {
                use ErrorCorrection::*;
                use QRCodeVersion::*;
                match ec {
                    L => impl_qr_version!(@impl fit count, $(,$version, $l )*),
                    M => impl_qr_version!(@impl fit count, $(,$version, $m )*),
                    Q => impl_qr_version!(@impl fit count, $(,$version, $q )*),
                    H => impl_qr_version!(@impl fit count, $(,$version, $h )*),
                }
            }
            /// Returns the specified number of codewords for a given
            /// version & [`ErrorCorrection`]
            #[inline]
            #[must_use]
            pub const fn data_size(self, ec: ErrorCorrection) -> usize {
                use ErrorCorrection::*;
                use QRCodeVersion::*;
                match ec {
                    L => match self { $( $version => $l, )* },
                    M => match self { $( $version => $m, )* },
                    Q => match self { $( $version => $q, )* },
                    H => match self { $( $version => $h, )* },
                }
            }
        }
    };
    (@impl fit $count: ident, $(,$version: ident, $ec: expr)*) => {
        match $count {
            $(
                ..=$ec => Some($version),
            )*
            _ => None
        }
    };
}

impl_qr_version! {
    (V1, 19, 16, 13, 9),
    (V2, 34, 28, 22, 16),
    (V3, 55, 44, 34, 26),
    (V4, 80, 64, 48, 36),
    (V5, 108, 86, 62, 46),
    (V6, 136, 108, 76, 60),
    (V7, 156, 124, 88, 66),
    (V8, 194, 154, 110, 86),
    (V9, 232, 182, 132, 100),
    (V10, 274, 216, 154, 122),
    (V11, 324, 254, 180, 140),
    (V12, 370, 290, 206, 158),
    (V13, 428, 334, 244, 180),
    (V14, 461, 365, 261, 197),
    (V15, 523, 415, 295, 223),
    (V16, 589, 453, 325, 253),
    (V17, 647, 507, 367, 283),
    (V18, 721, 563, 397, 313),
    (V19, 795, 627, 445, 341),
    (V20, 861, 669, 485, 385),
    (V21, 932, 714, 512, 406),
    (V22, 1006, 782, 568, 442),
    (V23, 1094, 860, 614, 464),
    (V24, 1174, 914, 664, 514),
    (V25, 1276, 1000, 718, 538),
    (V26, 1370, 1062, 754, 596),
    (V27, 1468, 1128, 808, 628),
    (V28, 1531, 1193, 871, 661),
    (V29, 1631, 1267, 911, 701),
    (V30, 1735, 1373, 985, 745),
    (V31, 1843, 1455, 1033, 793),
    (V32, 1955, 1541, 1115, 845),
    (V33, 2071, 1631, 1171, 901),
    (V34, 2191, 1725, 1231, 961),
    (V35, 2306, 1812, 1286, 986),
    (V36, 2434, 1914, 1354, 1054),
    (V37, 2566, 1992, 1426, 1096),
    (V38, 2702, 2102, 1502, 1142),
    (V39, 2812, 2216, 1582, 1222),
    (V40, 2956, 2334, 1666, 1276)
}


// total bits:
// 26
// 44
// 70
// 100
// 134
// 172
// 196
// 242
// 292
// 346
// 404
// 466
// 532
// 581
// 655
// 733
// 815
// 901
// 991
// 1085
// 1156
// 1258
// 1364
// 1474
// 1588
// 1706
// 1828
// 1921
// 2051
// 2185
// 2323
// 2465
// 2611
// 2761
// 2876
// 3034
// 3196
// 3362
// 3532
// 4706
