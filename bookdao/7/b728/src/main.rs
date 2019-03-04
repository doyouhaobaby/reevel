fn main() {
    //println!("Hello, world!");
    let e = E::Bar(PrintDrop("a"), PrintDrop("b"));

    println!("{:?}", e);

    let a = Foo {x: PrintDrop("foo1"), y:PrintDrop("foo2"), z:PrintDrop("foo3")};
}

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Drop {}", self.0);
    }
}

#[derive(Debug)]
struct PrintDrop(&'static str);

#[derive(Debug)]
enum E {
    Bar(PrintDrop, PrintDrop)
}

#[derive(Debug)]
struct Foo {
    x: PrintDrop,
    y: PrintDrop,
    z: PrintDrop,
}
