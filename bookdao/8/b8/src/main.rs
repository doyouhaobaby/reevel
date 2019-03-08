fn main() {
    let str = "hello";
    let mut chars = str.chars();
    assert_eq!(Some('h'), chars.next());
    assert_eq!(Some('e'), chars.next());
    assert_eq!(Some('l'), chars.next());
    assert_eq!(Some('l'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(None, chars.next());

    let mut bytes = str.bytes();
    assert_eq!(Some(104), bytes.next());
    assert_eq!(Some(101), bytes.next());
    assert_eq!(Some(108), bytes.next());
    assert_eq!(Some(108), bytes.next());
    assert_eq!(Some(111), bytes.next());
    assert_eq!(None, bytes.next());
}
