fn main() {
    let a = [1, 2, 3];
    let mut b = a.into_iter().map(|i| i*2);
    assert_eq!(Some(2), b.next());
    assert_eq!(Some(4), b.next());
    assert_eq!(Some(6), b.next());
    assert_eq!(None, b.next());
}
