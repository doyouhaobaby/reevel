fn main() {
    //println!("Hello, world!");
    let a = PrintDrop("hello world");
    println!("{:?}", a);

    let a = PrintDrop("hello world2");
    println!("{:?}", a);

    let _a = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
    let _a = (PrintDrop("d"), PrintDrop("e"), PrintDrop("f"));

    let c = (PrintDrop("c1"), PrintDrop("c2"), panic!()); 
}

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Drop {}", self.0);
    }
}

#[derive(Debug)]
struct PrintDrop(&'static str);
