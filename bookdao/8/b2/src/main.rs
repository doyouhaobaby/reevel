fn main() {
    let liu = '刘';
    let liu_u32 = liu as u32;
    //println!("{}", liu_u32);
    assert_eq!(liu_u32, 21016);

    println!("hex 0x{:x}", liu_u32);
    println!("unicode {}", liu.escape_unicode());
    //assert_eq!("\u{5212}", liu.escape_unicode()); 不能比较

    assert_eq!(char::from(32), ' ');
    assert_eq!(char::from(64), '@');
    assert_eq!(std::char::from_u32(21016),     Some(liu));
    assert_eq!(std::char::from_u32(0x5218), Some(liu));
    assert_eq!(std::char::from_u32(12911111), None);
}
