fn main() {
    let a = [1, 2, 3];
    let mut iter = a.iter().rev();
    println!("{:?}", iter);
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}
