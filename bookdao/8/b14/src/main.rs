fn main() {
    let mut a = "hello".to_string();
    let b = " world";
    let mut a = a + b;
    assert_eq!(a, "hello world");

    a += "!";
    assert_eq!(a, "hello world!");
}
