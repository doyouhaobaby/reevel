use std::str;

fn main() {
    let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", tao);

    assert_eq!("道", String::from("\u{9053}"));

    let unicode_x = 0x9053;
    println!("unicode x {:b}", unicode_x);

    let utf_x_hex = 0xe98193;
    println!("utf x {:b}", utf_x_hex);

    let utf_x_bin = 0b111010011000000110010011;
    println!("utf x bin {:x}", utf_x_bin);
}
