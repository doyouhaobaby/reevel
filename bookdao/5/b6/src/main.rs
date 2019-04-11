#[derive(Debug)]
struct A {
    x: i32,
    y: u32,
}

fn main() {
    let x = A {x: 5, y: 10};
    println!("x {:?}", x);
    let y = x;
    println!("y {:?}", x);
}
