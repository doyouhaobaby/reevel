#[derive(Debug,Clone,Copy)]
struct A {
    x: i32,
    y: Box<i32>,// this field does not implement `Copy`
}

fn main() {
}
