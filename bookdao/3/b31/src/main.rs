trait Page {
    fn set_page(&self, page: i32) {
        println!("set page {}", page);
    }
}

trait PerPage {
    fn set_pagesize(&self, page_size: i32) {
        println!("set page size {}", page_size);
    }
}

#[derive(Debug)]
struct MyPage {
    page: i32,
}

impl Page for MyPage {}

impl PerPage for MyPage {}

fn main() {
    let foo = MyPage {page: 5};
    println!("source {:?}", foo);
    foo.set_page(5);
    foo.set_pagesize(55);
}
