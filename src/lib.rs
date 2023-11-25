//! yeah
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

pub mod byte_stream;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QRCodeVersion {
    /// Total bytes: 26
    V1,
    /// Total bytes: 44
    V2,
    /// Total bytes: 70
    V3,
    /// Total bytes: 100
    V4,
    /// Total bytes: 134
    V5,
    /// Total bytes: 172
    V6,
    /// Total bytes: 196
    V7,
    /// Total bytes: 242
    V8,
    /// Total bytes: 292
    V9,
    /// Total bytes: 346
    V10,
    /// Total bytes: 404
    V11,
    /// Total bytes: 466
    V12,
    /// Total bytes: 532
    V13,
    /// Total bytes: 581
    V14,
    /// Total bytes: 655
    V15,
    /// Total bytes: 733
    V16,
    /// Total bytes: 815
    V17,
    /// Total bytes: 901
    V18,
    /// Total bytes: 991
    V19,
    /// Total bytes: 1085
    V20,
    /// Total bytes: 1156
    V21,
    /// Total bytes: 1258
    V22,
    /// Total bytes: 1364
    V23,
    /// Total bytes: 1474
    V24,
    /// Total bytes: 1588
    V25,
    /// Total bytes: 1706
    V26,
    /// Total bytes: 1828
    V27,
    /// Total bytes: 1921
    V28,
    /// Total bytes: 2051
    V29,
    /// Total bytes: 2185
    V30,
    /// Total bytes: 2323
    V31,
    /// Total bytes: 2465
    V32,
    /// Total bytes: 2611
    V33,
    /// Total bytes: 2761
    V34,
    /// Total bytes: 2876
    V35,
    /// Total bytes: 3034
    V36,
    /// Total bytes: 3196
    V37,
    /// Total bytes: 3362
    V38,
    /// Total bytes: 3532
    V39,
    /// Total bytes: 4706
    V40,
}
/// Designates the way a block is split up
///
/// Number of error correction bytes is held within its container, [`GroupSplit`]
#[derive(Debug)]
pub struct BlockSplit {
    /// The number of data bytes per block
    pub data_bytes: usize,
    /// The total number of blocks
    pub blocks: usize,
}
/// Designates the number and structure of groups
#[derive(Debug)]
pub enum GroupSplit {
    /// The division of blocks when only one group is needed
    One {
        ec_bytes: usize,
        block_split: BlockSplit,
    },
    /// The division of blocks when two groups are needed
    Two {
        ec_bytes: usize,
        block_split_one: BlockSplit,
        block_split_two: BlockSplit,
    },
}

macro_rules! impl_qr {
    (
        $(
            (
                $version: ident,
                ($data_l: literal, $ec_l: literal,
                    [   ($g_l_1_d: literal, $g_l_1_b: literal)
                    $(, ($g_l_2_d: literal, $g_l_2_b: literal) )?]
                ),
                ($data_m: literal, $ec_m: literal,
                    [   ($g_m_1_d: literal, $g_m_1_b: literal)
                    $(, ($g_m_2_d: literal, $g_m_2_b: literal) )?]
                ),
                ($data_q: literal, $ec_q: literal,
                    [   ($g_q_1_d: literal, $g_q_1_b: literal)
                    $(, ($g_q_2_d: literal, $g_q_2_b: literal) )?]
                ),
                ($data_h: literal, $ec_h: literal,
                    [   ($g_h_1_d: literal, $g_h_1_b: literal)
                    $(, ($g_h_2_d: literal, $g_h_2_b: literal) )?]
                )
            )
        ),*
    ) => {
        impl_qr![@impl sizes $( $version, $data_l, $data_m, $data_q, $data_h ),+];
        impl_qr![@impl GroupSplit $(
            $version,
            $data_l, $ec_l,
            $data_m, $ec_m,
            $data_q, $ec_q,
            $data_h, $ec_h,
                ($g_l_1_d, $g_l_1_b),
             $(,($g_l_2_d, $g_l_2_b))?
                ($g_m_1_d, $g_m_1_b),
             $(,($g_m_2_d, $g_m_2_b))?
                ($g_q_1_d, $g_q_1_b),
             $(,($g_q_2_d, $g_q_2_b))?
                ($g_h_1_d, $g_h_1_b),
             $(,($g_h_2_d, $g_h_2_b))?
        ),+ ];
    };
    (@impl GroupSplit $(
        $version: ident,
        $data_l: literal, $ec_l: literal,
        $data_m: literal, $ec_m: literal,
        $data_q: literal, $ec_q: literal,
        $data_h: literal, $ec_h: literal,
        ($g_l_1_d: literal, $g_l_1_b: literal),
        $(, ($g_l_2_d: literal, $g_l_2_b: literal) )?
        ($g_m_1_d: literal, $g_m_1_b: literal),
        $(, ($g_m_2_d: literal, $g_m_2_b: literal) )?
        ($g_q_1_d: literal, $g_q_1_b: literal),
        $(, ($g_q_2_d: literal, $g_q_2_b: literal) )?
        ($g_h_1_d: literal, $g_h_1_b: literal),
        $(, ($g_h_2_d: literal, $g_h_2_b: literal) )?
    ),+) => {
        /// Returns the way blocks should be split according to the given
        /// version and error correction level
        #[inline]
        #[must_use]
        pub const fn split(self, ec: ErrorCorrection) -> GroupSplit {
            use GroupSplit::*;
            use QRCodeVersion::*;
            use ErrorCorrection::*;
            match self {
                $(
                    $version => match ec {
                        L => impl_qr! [@impl group $ec_l,
                                $g_l_1_d, $g_l_1_b
                            $(, $g_l_2_d, $g_l_2_b )?
                        ],
                        M => impl_qr! [@impl group $ec_m,
                                $g_m_1_d, $g_m_1_b
                            $(, $g_m_2_d, $g_m_2_b )?
                        ],
                        Q => impl_qr! [@impl group $ec_q,
                                $g_q_1_d, $g_q_1_b
                            $(, $g_q_2_d, $g_q_2_b )?
                        ],
                        H => impl_qr! [@impl group $ec_h,
                                $g_h_1_d, $g_h_1_b
                            $(, $g_h_2_d, $g_h_2_b )?
                        ],
                    },
                )+
            }
        }
    };
    (@impl group $ec_bytes: literal,
        $data_bytes_one: literal, $blocks_one: literal,
        $data_bytes_two: literal, $blocks_two: literal
    ) => {
        Two {
            ec_bytes: $ec_bytes,
            block_split_one: BlockSplit {
                data_bytes: $data_bytes_one,
                blocks: $blocks_one
            },
            block_split_two: BlockSplit {
                data_bytes: $data_bytes_two,
                blocks: $blocks_two
            },
        }
    };
    (@impl group $ec_bytes: literal,
        $data_bytes: literal,
        $blocks: literal
    ) => {
        One {
            ec_bytes: $ec_bytes,
            block_split: BlockSplit {
                data_bytes: $data_bytes,
                blocks: $blocks
            }
        }
    };
    (@impl sizes
        $(
            $version: ident,
            $l: literal,
            $m: literal,
            $q: literal,
            $h: literal
        ),+
    ) => {
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
        /// Fits a given number of codewords and [`ErrorCorrection`]
        ///
        #[inline]
        #[must_use]
        pub const fn fit_version(count: usize, ec: ErrorCorrection) -> Option<Self> {
            use ErrorCorrection::*;
            use QRCodeVersion::*;
            match ec {
                L => impl_qr!(@impl fit_version count, $(,$version, $l )*),
                M => impl_qr!(@impl fit_version count, $(,$version, $m )*),
                Q => impl_qr!(@impl fit_version count, $(,$version, $q )*),
                H => impl_qr!(@impl fit_version count, $(,$version, $h )*),
            }
        }
    };
    (@impl fit_version $count: ident, $(,$version: ident, $ec: expr)*) => {
        match $count {
            $(
                ..=$ec => Some($version),
            )*
            _ => None
        }
    };
}
impl QRCodeVersion {
    // Could make this an enum instead?
    impl_qr![
        (
            V1,
            (19, 7, [(1, 19)]),
            (16, 10, [(1, 16)]),
            (13, 13, [(1, 13)]),
            (9, 17, [(1, 9)])
        ),
        (
            V2,
            (34, 10, [(1, 34)]),
            (28, 16, [(1, 28)]),
            (22, 22, [(1, 22)]),
            (16, 28, [(1, 16)])
        ),
        (
            V3,
            (55, 15, [(1, 55)]),
            (44, 26, [(1, 44)]),
            (34, 18, [(2, 17)]),
            (26, 22, [(2, 13)])
        ),
        (
            V4,
            (80, 20, [(1, 80)]),
            (64, 18, [(2, 32)]),
            (48, 26, [(2, 24)]),
            (36, 16, [(4, 9)])
        ),
        (
            V5,
            (108, 26, [(1, 108)]),
            (86, 24, [(2, 43)]),
            (62, 18, [(2, 15), (2, 16)]),
            (46, 22, [(2, 11), (2, 12)])
        ),
        (
            V6,
            (136, 18, [(2, 68)]),
            (108, 16, [(4, 27)]),
            (76, 24, [(4, 19)]),
            (60, 28, [(4, 15)])
        ),
        (
            V7,
            (156, 20, [(2, 78)]),
            (124, 18, [(4, 31)]),
            (88, 18, [(2, 14), (4, 15)]),
            (66, 26, [(4, 13), (1, 14)])
        ),
        (
            V8,
            (194, 24, [(2, 97)]),
            (154, 22, [(2, 38), (2, 39)]),
            (110, 22, [(4, 18), (2, 19)]),
            (86, 26, [(4, 14), (2, 15)])
        ),
        (
            V9,
            (232, 30, [(2, 116)]),
            (182, 22, [(3, 36), (2, 37)]),
            (132, 20, [(4, 16), (4, 17)]),
            (100, 24, [(4, 12), (4, 13)])
        ),
        (
            V10,
            (274, 18, [(2, 68), (2, 69)]),
            (216, 26, [(4, 43), (1, 44)]),
            (154, 24, [(6, 19), (2, 20)]),
            (122, 28, [(6, 15), (2, 16)])
        ),
        (
            V11,
            (324, 20, [(4, 81)]),
            (254, 30, [(1, 50), (4, 51)]),
            (180, 28, [(4, 22), (4, 23)]),
            (140, 24, [(3, 12), (8, 13)])
        ),
        (
            V12,
            (370, 24, [(2, 92), (2, 93)]),
            (290, 22, [(6, 36), (2, 37)]),
            (206, 26, [(4, 20), (6, 21)]),
            (158, 28, [(7, 14), (4, 15)])
        ),
        (
            V13,
            (428, 26, [(4, 107)]),
            (334, 22, [(8, 37), (1, 38)]),
            (244, 24, [(8, 20), (4, 21)]),
            (180, 22, [(12, 11), (4, 12)])
        ),
        (
            V14,
            (461, 30, [(3, 115), (1, 116)]),
            (365, 24, [(4, 40), (5, 41)]),
            (261, 20, [(11, 16), (5, 17)]),
            (197, 24, [(11, 12), (5, 13)])
        ),
        (
            V15,
            (523, 22, [(5, 87), (1, 88)]),
            (415, 24, [(5, 41), (5, 42)]),
            (295, 30, [(5, 24), (7, 25)]),
            (223, 24, [(11, 12), (7, 13)])
        ),
        (
            V16,
            (589, 24, [(5, 98), (1, 99)]),
            (453, 28, [(7, 45), (3, 46)]),
            (325, 24, [(15, 19), (2, 20)]),
            (253, 30, [(3, 15), (13, 16)])
        ),
        (
            V17,
            (647, 28, [(1, 107), (5, 108)]),
            (507, 28, [(10, 46), (1, 47)]),
            (367, 28, [(1, 22), (15, 23)]),
            (283, 28, [(2, 14), (17, 15)])
        ),
        (
            V18,
            (721, 30, [(5, 120), (1, 121)]),
            (563, 26, [(9, 43), (4, 44)]),
            (397, 28, [(17, 22), (1, 23)]),
            (313, 28, [(2, 14), (19, 15)])
        ),
        (
            V19,
            (795, 28, [(3, 113), (4, 114)]),
            (627, 26, [(3, 44), (11, 45)]),
            (445, 26, [(17, 21), (4, 22)]),
            (341, 26, [(9, 13), (16, 14)])
        ),
        (
            V20,
            (861, 28, [(3, 107), (5, 108)]),
            (669, 26, [(3, 41), (13, 42)]),
            (485, 30, [(15, 24), (5, 25)]),
            (385, 28, [(15, 15), (10, 16)])
        ),
        (
            V21,
            (932, 28, [(4, 116), (4, 117)]),
            (714, 26, [(17, 42)]),
            (512, 28, [(17, 22), (6, 23)]),
            (406, 30, [(19, 16), (6, 17)])
        ),
        (
            V22,
            (1006, 28, [(2, 111), (7, 112)]),
            (782, 28, [(17, 46)]),
            (568, 30, [(7, 24), (16, 25)]),
            (442, 24, [(34, 13)])
        ),
        (
            V23,
            (1094, 30, [(4, 121), (5, 122)]),
            (860, 28, [(4, 47), (14, 48)]),
            (614, 30, [(11, 24), (14, 25)]),
            (464, 30, [(16, 15), (14, 16)])
        ),
        (
            V24,
            (1174, 30, [(6, 117), (4, 118)]),
            (914, 28, [(6, 45), (14, 46)]),
            (664, 30, [(11, 24), (16, 25)]),
            (514, 30, [(30, 16), (2, 17)])
        ),
        (
            V25,
            (1276, 26, [(8, 106), (4, 107)]),
            (1000, 28, [(8, 47), (13, 48)]),
            (718, 30, [(7, 24), (22, 25)]),
            (538, 30, [(22, 15), (13, 16)])
        ),
        (
            V26,
            (1370, 28, [(10, 114), (2, 115)]),
            (1062, 28, [(19, 46), (4, 47)]),
            (754, 28, [(28, 22), (6, 23)]),
            (596, 30, [(33, 16), (4, 17)])
        ),
        (
            V27,
            (1468, 30, [(8, 122), (4, 123)]),
            (1128, 28, [(22, 45), (3, 46)]),
            (808, 30, [(8, 23), (26, 24)]),
            (628, 30, [(12, 15), (28, 16)])
        ),
        (
            V28,
            (1531, 30, [(3, 117), (10, 118)]),
            (1193, 28, [(3, 45), (23, 46)]),
            (871, 30, [(4, 24), (31, 25)]),
            (661, 30, [(11, 15), (31, 16)])
        ),
        (
            V29,
            (1631, 30, [(7, 116), (7, 117)]),
            (1267, 28, [(21, 45), (7, 46)]),
            (911, 30, [(1, 23), (37, 24)]),
            (701, 30, [(19, 15), (26, 16)])
        ),
        (
            V30,
            (1735, 30, [(5, 115), (10, 116)]),
            (1373, 28, [(19, 47), (10, 48)]),
            (985, 30, [(15, 24), (25, 25)]),
            (745, 30, [(23, 15), (25, 16)])
        ),
        (
            V31,
            (1843, 30, [(13, 115), (3, 116)]),
            (1455, 28, [(2, 46), (29, 47)]),
            (1033, 30, [(42, 24), (1, 25)]),
            (793, 30, [(23, 15), (28, 16)])
        ),
        (
            V32,
            (1955, 30, [(17, 115)]),
            (1541, 28, [(10, 46), (23, 47)]),
            (1115, 30, [(10, 24), (35, 25)]),
            (845, 30, [(19, 15), (35, 16)])
        ),
        (
            V33,
            (2071, 30, [(17, 115), (1, 116)]),
            (1631, 28, [(14, 46), (21, 47)]),
            (1171, 30, [(29, 24), (19, 25)]),
            (901, 30, [(11, 15), (46, 16)])
        ),
        (
            V34,
            (2191, 30, [(13, 115), (6, 116)]),
            (1725, 28, [(14, 46), (23, 47)]),
            (1231, 30, [(44, 24), (7, 25)]),
            (961, 30, [(59, 16), (1, 17)])
        ),
        (
            V35,
            (2306, 30, [(12, 121), (7, 122)]),
            (1812, 28, [(12, 47), (26, 48)]),
            (1286, 30, [(39, 24), (14, 25)]),
            (986, 30, [(22, 15), (41, 16)])
        ),
        (
            V36,
            (2434, 30, [(6, 121), (14, 122)]),
            (1914, 28, [(6, 47), (34, 48)]),
            (1354, 30, [(46, 24), (10, 25)]),
            (1054, 30, [(2, 15), (64, 16)])
        ),
        (
            V37,
            (2566, 30, [(17, 122), (4, 123)]),
            (1992, 28, [(29, 46), (14, 47)]),
            (1426, 30, [(49, 24), (10, 25)]),
            (1096, 30, [(24, 15), (46, 16)])
        ),
        (
            V38,
            (2702, 30, [(4, 122), (18, 123)]),
            (2102, 28, [(13, 46), (32, 47)]),
            (1502, 30, [(48, 24), (14, 25)]),
            (1142, 30, [(42, 15), (32, 16)])
        ),
        (
            V39,
            (2812, 30, [(20, 117), (4, 118)]),
            (2216, 28, [(40, 47), (7, 48)]),
            (1582, 30, [(43, 24), (22, 25)]),
            (1222, 30, [(10, 15), (67, 16)])
        ),
        (
            V40,
            (2956, 30, [(19, 118), (6, 119)]),
            (2334, 28, [(18, 47), (31, 48)]),
            (1666, 30, [(34, 24), (34, 25)]),
            (1276, 30, [(20, 15), (61, 16)])
        )
    ];
}
 