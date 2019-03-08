fn main() {
    let mut v = String::from("hello world");
    assert_eq!(Some("h"), v.get(0..1));
    assert_eq!(Some("e"), v.get(1..2));
    assert_eq!(Some("ello world"), v.get(1..));
    assert_eq!(Some("he"), v.get(..2));
    assert!(v.is_char_boundary(4));
    let m = v.get_mut(6..9);
    println!("hello {:?}", m);
    if let Some(x) = m {
        println!("xxx {}", x);
    }
    assert!(v.get_mut(..444).is_none());
    assert!(!v.get_mut(3..).is_none());
}
