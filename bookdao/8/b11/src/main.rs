fn main() {
    let mut a = String::from("hello,");
    a.push('w');
    assert_eq!(a, "hello,w");
    a.push_str("orld");
    assert_eq!(a, "hello,world");
}
