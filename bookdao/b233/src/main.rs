fn foo (a: (i32, i32)) -> (i32, i32) {
    (a.0 + 1, a.1 + 1)
} 

fn main() {
    let a: (&'static str, i32, char) = ("hello world", 5, 'c');
    assert_eq!(a.0, "hello world");
    assert_eq!(a.1, 5);
    assert_eq!(a.2, 'c');

    let b = (5, 4);
    assert_eq!(foo((5, 4)), (6, 5));
    assert_eq!(foo(b), (6,5));

    let (b,c) = (7, 8);
    assert_eq!(7, b);
    assert_eq!(8, c);
}
