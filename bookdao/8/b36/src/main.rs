fn main() {
    assert_eq!(format!("{:+}", 1234), "+1234");
    assert_eq!(format!("{:+x}", 1234), "+4d2");
    assert_eq!(format!("{:+#x}", 1234), "+0x4d2");
    assert_eq!(format!("{:b}", 1234), "10011010010");
    assert_eq!(format!("{:#b}", 1234), "0b10011010010");
    assert_eq!(format!("{:#20b}", 1234), "       0b10011010010");
    assert_eq!(format!("{:+#b}", 1234), "+0b10011010010");
    
}
