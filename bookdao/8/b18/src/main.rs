fn main() {
    let a = "hello world";
    assert!(a.contains('h'));
    assert!(a.contains("w"));
    assert!(a.contains("worl"));
    assert!(a.starts_with("h"));
    assert!(a.starts_with('h'));
    assert!(a.ends_with("rld"));
}
