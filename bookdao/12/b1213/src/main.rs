fn main() {
    let x = Foo::new();
    assert_eq!(x, Foo);
}
#[derive(new)]
pub struct Foo;

