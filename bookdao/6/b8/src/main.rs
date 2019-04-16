fn foo (x: isize, y: isize) -> (isize, isize) {
    (x-y, x+y)
}

fn main() {
    let (a, b) = foo(3, 5);
    assert_eq!(-2, a);
    assert_eq!(8, b);
}
