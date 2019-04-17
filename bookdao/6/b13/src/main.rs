// fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
//     op(a, b)
// }

fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32)-> i32 {
    a * b
}

fn main() {
    let (a, b) = (2, 4);
    assert_eq!(math(sum, a, b), 6);
    assert_eq!(math(product, a, b), 8);
}
