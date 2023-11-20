// use crate::{
//     encoding::{encode_alphanumeric, encode_byte, encode_numeric},
//     ErrorCorrection, QRCodeVersion,
// };

// #[macro_export]
// macro_rules! test {
//     (
//         $version: expr,
//         $ec: expr,
//         $encode: expr,
//         $(
//             $s: expr,
//             $a: tt
//             // [$($a: expr),*]
//         ),+
//     ) => {
//         $(
//             assert_eq!(
//                 $encode($s, $version, $ec),
//                 $a,
//                 "{}",
//                 $s,
//             );
//         )+
//     };
// }

// #[test]
// fn numeric() {
//     test![
//         QRCodeVersion::V1,
//         ErrorCorrection::H,
//         encode_numeric,
//         "9",
//         [0x10, 0x06, 0x40, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11],
//         "99",
//         [0x10, 0x0B, 0x18, 0x00, 0xEC, 0x11, 0xEC, 0x11, 0xEC],
//         "999",
//         [0x10, 0x0F, 0xE7, 0x00, 0xEC, 0x11, 0xEC, 0x11, 0xEC],
//         "9999",
//         [0x10, 0x13, 0xE7, 0x90, 0xEC, 0x11, 0xEC, 0x11, 0xEC],
//         "99999",
//         [0x10, 0x17, 0xE7, 0xC6, 0x00, 0xEC, 0x11, 0xEC, 0x11],
//         "999999",
//         [0x10, 0x1B, 0xE7, 0xF9, 0xC0, 0xEC, 0x11, 0xEC, 0x11],
//         "9999999",
//         [0x10, 0x1F, 0xE7, 0xF9, 0xE4, 0x00, 0xEC, 0x11, 0xEC],
//         "99999999",
//         [0x10, 0x23, 0xE7, 0xF9, 0xF1, 0x80, 0xEC, 0x11, 0xEC],
//         "999999999",
//         [0x10, 0x27, 0xE7, 0xF9, 0xFE, 0x70, 0xEC, 0x11, 0xEC],
//         "99999999999999999",
//         [0x10, 0x47, 0xE7, 0xF9, 0xFE, 0x7F, 0x9F, 0xE7, 0xC6]
//     ];
//     test![
//         QRCodeVersion::V3,
//         ErrorCorrection::H,
//         encode_numeric,
//         "31415926535897932384626433832795028841971693993",
//         [
//             0x10, 0xBD, 0x3A, 0x27, 0xD0, 0x95, 0x9B, 0xD3, 0x50, 0xF4, 0xE4, 0x21, 0x52, 0x51,
//             0xFB, 0x64, 0x81, 0xA3, 0xB3, 0x3A, 0xBB, 0xA0, 0xEC, 0x11, 0xEC, 0x11
//         ],
//         "97741478051252836695031785333702546407524468981",
//         [
//             0x10, 0xBF, 0xD1, 0x67, 0xB0, 0xC8, 0x02, 0x10, 0x5B, 0xBB, 0x64, 0xF7, 0x55, 0x54,
//             0x41, 0x97, 0x40, 0x4B, 0x3D, 0x2B, 0x1A, 0x20, 0xEC, 0x11, 0xEC, 0x11
//         ],
//         "65171026260096390703087863992791699776343165894",
//         [
//             0x10, 0xBE, 0x8B, 0xB1, 0x90, 0x69, 0x63, 0xC3, 0xE2, 0xC1, 0xED, 0xBA, 0x7F, 0xE7,
//             0xF9, 0x4F, 0x96, 0xFB, 0x6B, 0xE9, 0x2B, 0xC0, 0xEC, 0x11, 0xEC, 0x11
//         ]
//     ];
// }

// #[test]
// fn alphanumeric() {
//     test![
//         QRCodeVersion::V4,
//         ErrorCorrection::H,
//         encode_alphanumeric,
//         "HELLO WORLD",
//         [
//             0x20, 0x5B, 0x0B, 0x78, 0xD1, 0x72, 0xDC, 0x4D, 0x43, 0x40, 0xEC, 0x11, 0xEC, 0x11,
//             0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11,
//             0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11
//         ],
//         "PROJECT NAYUKI",
//         [
//             0x20, 0x74, 0x80, 0x89, 0x6A, 0x0A, 0x9E, 0xC1, 0x5C, 0x30, 0xE5, 0x80, 0xEC, 0x11,
//             0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11,
//             0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11
//         ],
//         "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:",
//         [
//             0x21, 0x68, 0x01, 0x0B, 0xA2, 0xE4, 0x8A, 0x97, 0x13, 0x9A, 0x8A, 0x54, 0x2A, 0xE1,
//             0x67, 0xAE, 0x65, 0xFA, 0xC5, 0x19, 0x5B, 0x42, 0x6B, 0x2D, 0xC1, 0xC3, 0xB9, 0xE7,
//             0x6A, 0xF3, 0x1F, 0x1B, 0x60, 0x00, 0xEC, 0x11
//         ]
//     ];
// }
// #[test]
// fn byte() {
//     test![
//         QRCodeVersion::V8,
//         ErrorCorrection::H,
//         encode_byte,
//         "अMޞ▉c𙩐4٥񁕃󶾇򍛇ϒ̳nȀ󏞎{𶤺̞♝ߖ臶ݹ鞜퀢ᄉAͳ򓿟迤",
//         [
//             0x44, 0xFE, 0x0A, 0x48, 0x54, 0xDD, 0xE9, 0xEE, 0x29, 0x68, 0x96, 0x3F, 0x09, 0x9A,
//             0x99, 0x03, 0x4D, 0x9A, 0x5F, 0x18, 0x19, 0x58, 0x3F, 0x3B, 0x6B, 0xE8, 0x7F, 0x28,
//             0xD9, 0xB8, 0x7C, 0xF9, 0x2C, 0xCB, 0x36, 0xEC, 0x88, 0x0F, 0x38, 0xF9, 0xE8, 0xE7,
//             0xBF, 0x0B, 0x6A, 0x4B, 0xAC, 0xC9, 0xEE, 0x29, 0x99, 0xDD, 0xF9, 0x6E, 0x88, 0x7B,
//             0x6E, 0xE8, 0xB9, 0x1D, 0xDB, 0x9E, 0x99, 0xE9, 0xCE, 0xD8, 0x0A, 0x2E, 0x18, 0x48,
//             0x94, 0x1C, 0xDB, 0x3F, 0x29, 0x3B, 0xF9, 0xFE, 0x8B, 0xFA, 0x40, 0xEC, 0x11, 0xEC,
//             0x11, 0xEC
//         ],
//         "🔛 🕤 🎉 🔭 💿 🧀 🗺 💺 😲 📥 😗 🔶 🎬 🚘 🤕 😥 ",
//         [
//             0x45, 0x0F, 0x09, 0xF9, 0x49, 0xB2, 0x0F, 0x09, 0xF9, 0x5A, 0x42, 0x0F, 0x09, 0xF8,
//             0xE8, 0x92, 0x0F, 0x09, 0xF9, 0x4A, 0xD2, 0x0F, 0x09, 0xF9, 0x2B, 0xF2, 0x0F, 0x09,
//             0xFA, 0x78, 0x02, 0x0F, 0x09, 0xF9, 0x7B, 0xA2, 0x0F, 0x09, 0xF9, 0x2B, 0xA2, 0x0F,
//             0x09, 0xF9, 0x8B, 0x22, 0x0F, 0x09, 0xF9, 0x3A, 0x52, 0x0F, 0x09, 0xF9, 0x89, 0x72,
//             0x0F, 0x09, 0xF9, 0x4B, 0x62, 0x0F, 0x09, 0xF8, 0xEA, 0xC2, 0x0F, 0x09, 0xF9, 0xA9,
//             0x82, 0x0F, 0x09, 0xFA, 0x49, 0x52, 0x0F, 0x09, 0xF9, 0x8A, 0x52, 0x00, 0xEC, 0x11,
//             0xEC, 0x11
//         ],
//         "🔋 🌠 🆎 🌿 5️⃣ 🌐 👚 8️⃣ 💕 🚲 👵 4️⃣ 🌂 🍤 🔬 ",
//         [
//             0x45, 0x4F, 0x09, 0xF9, 0x48, 0xB2, 0x0F, 0x09, 0xF8, 0xCA, 0x02, 0x0F, 0x09, 0xF8,
//             0x68, 0xE2, 0x0F, 0x09, 0xF8, 0xCB, 0xF2, 0x03, 0x5E, 0xFB, 0x88, 0xFE, 0x28, 0x3A,
//             0x32, 0x0F, 0x09, 0xF8, 0xC9, 0x02, 0x0F, 0x09, 0xF9, 0x19, 0xA2, 0x03, 0x8E, 0xFB,
//             0x88, 0xFE, 0x28, 0x3A, 0x32, 0x0F, 0x09, 0xF9, 0x29, 0x52, 0x0F, 0x09, 0xF9, 0xAB,
//             0x22, 0x0F, 0x09, 0xF9, 0x1B, 0x52, 0x03, 0x4E, 0xFB, 0x88, 0xFE, 0x28, 0x3A, 0x32,
//             0x0F, 0x09, 0xF8, 0xC8, 0x22, 0x0F, 0x09, 0xF8, 0xDA, 0x42, 0x0F, 0x09, 0xF9, 0x4A,
//             0xC2, 0x00
//         ],
//         "Lorem ipsum dolor sit amet. Qui suscipit vitae sed tenetur amet et voluptas commodi ",
//         [
//             0x45, 0x44, 0xC6, 0xF7, 0x26, 0x56, 0xD2, 0x06, 0x97, 0x07, 0x37, 0x56, 0xD2, 0x06,
//             0x46, 0xF6, 0xC6, 0xF7, 0x22, 0x07, 0x36, 0x97, 0x42, 0x06, 0x16, 0xD6, 0x57, 0x42,
//             0xE2, 0x05, 0x17, 0x56, 0x92, 0x07, 0x37, 0x57, 0x36, 0x36, 0x97, 0x06, 0x97, 0x42,
//             0x07, 0x66, 0x97, 0x46, 0x16, 0x52, 0x07, 0x36, 0x56, 0x42, 0x07, 0x46, 0x56, 0xE6,
//             0x57, 0x47, 0x57, 0x22, 0x06, 0x16, 0xD6, 0x57, 0x42, 0x06, 0x57, 0x42, 0x07, 0x66,
//             0xF6, 0xC7, 0x57, 0x07, 0x46, 0x17, 0x32, 0x06, 0x36, 0xF6, 0xD6, 0xD6, 0xF6, 0x46,
//             0x92, 0x00
//         ]
//     ];
// }

// #[test]
// fn numeric_limit() {
//     let nines = "9".repeat(7089);
//     let mut encoded = vec![0x16, 0xEC, 0x7E];
//     encoded.append(&mut [0x7F, 0x9F, 0xE7, 0xF9, 0xFE].repeat(590));
//     encoded.extend([0x7F, 0x9F, 0xE7]);
//     test![
//         QRCodeVersion::V40,
//         ErrorCorrection::L,
//         encode_numeric,
//         &nines,
//         encoded
//     ];
// }

// #[test]
// fn alphanumeric_limit() {
//     let colons = ":".repeat(4296);
//     let mut encoded = vec![0x28, 0x64];
//     encoded.append(
//         &mut [
//             0x7E, 0x8F, 0xD1, 0xFA, 0x3F, 0x47, 0xE8, 0xFD, 0x1F, 0xA3, 0xF4,
//         ]
//         .repeat(268),
//     );
//     encoded.extend([0x7E, 0x8F, 0xD1, 0xFA, 0x3F, 0x40]);
//     test![
//         QRCodeVersion::V40,
//         ErrorCorrection::L,
//         encode_alphanumeric,
//         &colons,
//         encoded
//     ];
// }

// #[test]
// fn byte_limit() {
//     let chars = char::MAX.to_string().repeat(738);
//     let mut encoded = vec![0x40, 0xB8, 0x8F];
//     encoded.append(&mut [0x48, 0xFB, 0xFB, 0xFF].repeat(737));
//     encoded.extend([0x48, 0xFB, 0xFB, 0xF0, 0xEC]);
//     test![
//         QRCodeVersion::V40,
//         ErrorCorrection::L,
//         encode_byte,
//         &chars,
//         encoded
//     ];
// }
