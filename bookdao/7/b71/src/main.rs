#[derive(Debug)]
//#[derive(Debug,Copy,Clone)]
struct Book <'a> {
    name: &'a str,
    isbn: i32,
    version: i32,
}

fn main() {
    let book = Book {
        name: "rust book",
        isbn: 20181212,
        version: 1
    };

    println!("{:?}", book);

    let book2 = Book {version: 2 ,..book};

    println!("{:?}", book2);
}
