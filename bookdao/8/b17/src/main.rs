fn main() {
    let mut s = "hello world".to_string();
    s.remove(4);
    assert_eq!("hell world", s);
    assert_eq!(Some('d'), s.pop());
    assert_eq!(Some('l'), s.pop());
    assert_eq!(Some('r'), s.pop());

    let mut s = String::from("hello");
    s.truncate(3);
    assert_eq!("hel", s);
    s.clear();
    assert_eq!("", s);

    let mut s = String::from("hello | world");
    let p = s.find('|').unwrap_or(s.len());
    assert_eq!(p, 6);

    let t: String = s.drain(..p).collect();
    assert_eq!(t, "hello ");
    assert_eq!(s, "| world");
    s.drain(..);
    assert_eq!("", s);
}
