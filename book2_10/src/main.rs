fn sum (a: i32, b: i32) -> i32 {
    a + b
}

fn multipcation (a: i32, b: i32) -> i32 {
    a * b
}

fn demo (op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn main() {
    assert_eq!(15, sum(10, 5));
    assert_eq!(15, multipcation(3, 5));
    assert_eq!(15, demo(sum, 10, 5));
    assert_eq!(15, demo(multipcation, 3, 5));
}
