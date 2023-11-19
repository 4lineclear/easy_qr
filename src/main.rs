use easy_qr::{encoding::Encodable, ErrorCorrection, QRCodeVersion};

fn main() {
    // let mut bytes = Bytes::default();
    // bytes.push::<_16>(10);
    // test_numeric();
    // test_alphanumeric()
    // test_byte();
    // for _ in 0..4296 {
    //     print!(":")
    // }
    // println!()
}
#[inline]
fn test(s: &str) {
    let encoded = s.create_bits(QRCodeVersion::V2, ErrorCorrection::H);
    println!("{}:", s);
    encoded
        .0
        .into_iter()
        .for_each(|cw| println!("byte: {cw:#04x} - {cw:#010b}"));
}
#[inline]
pub fn test_byte() {
    for s in ["aЉ윇😱"] {
        test(s)
    }
}
#[inline]
pub fn test_alphanumeric() {
    for s in ["PROJECT NAYUKI"] {
        test(s)
    }
}
#[inline]
pub fn test_numeric() {
    for s in [
        "9",
        "99",
        "999",
        "9999",
        "99999",
        "999999",
        "9999999",
        "99999999",
        "999999999",
    ] {
        test(s)
    }
}
