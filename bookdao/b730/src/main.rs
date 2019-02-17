fn main() {
    let a = PrintDrop("a");
    let b = PrintDrop("b");
    let c = PrintDrop("c");

    let d = move || {a;c;b;};
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
