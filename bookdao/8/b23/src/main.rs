fn main() {
    let s = " hello world \t ";
    assert_eq!("hello world", s.trim());
    assert_eq!("hello world \t ", s.trim_start());
    assert_eq!(" hello world", s.trim_end());
}
