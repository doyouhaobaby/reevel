fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut iter = numbers.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), Some(8));
    assert_eq!(iter.next_back(), Some(7));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}
