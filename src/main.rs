use easy_qr::{encoding::Encodable, ErrorCorrection, QRCodeVersion};

fn main() {
    // test_numeric();
    test_alphanumeric()
}
#[inline]
fn test(s: &str) {
    let encoded = s.create_bits(QRCodeVersion::V1, ErrorCorrection::H);
    println!("{} {}:", s, s.len().div_ceil(3));
    encoded
        .0
        .into_iter()
        .for_each(|cw| println!("byte: {cw:#04x} - {cw:#010b}"));
}
#[inline]
fn test_alphanumeric() {
    for s in [
        "PROJECT NAYUKI",
    ] {
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
