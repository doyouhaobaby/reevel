fn main() {
    let a = PrintDrop("a");
    let b = PrintDrop("b");
    let c = PrintDrop("c");

    let d = move || {
        {
            let z = &b;// 先被借用，释放后才开始 move 到闭包，b 首先被释放
        }
        a;b;c;
    };
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
