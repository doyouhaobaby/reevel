fn main() {
    let a = "hello";
    let b = " world";
    let c = a.to_string() + b;
    assert_eq!("hello world", c);
}
