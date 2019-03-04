fn main() {
    let a = true;
    let mut b = 1;
    assert_eq!(1, b);

    if let true = a {
        b = 3
    }

    assert_eq!(b, 3);
}
