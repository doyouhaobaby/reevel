use std::cell::Cell;
struct Foo {
    x: u32,
    y: Cell<u32>,
}

fn main() {
    let foo = Foo {x: 1, y: Cell::new(3)};
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(6);
    assert_eq!(6, foo.y.get());
}
