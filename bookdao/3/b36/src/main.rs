#[derive(Debug)]
struct Foo;

trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("foo {:?}", self);
    }
}

fn static_dispatch<T> (t: &T) where T:Bar {
    t.baz();
}

fn dynamic_dispatch(t: &Bar) {
    t.baz();
}

fn main() {
    let f = Foo;
    f.baz();
    static_dispatch(&f);
    dynamic_dispatch(&f);
}
