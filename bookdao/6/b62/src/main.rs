fn main() {
    let a : [i32; 3] = [4, 5, 6];
    let mut iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());
    iter.next();
    assert_eq!((2, Some(2)), iter.size_hint());
}
