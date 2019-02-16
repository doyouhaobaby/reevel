fn main() {
    println!("Hello, world!");

    let a = Option::Some(5);

    println!("{:?}", a);
}

#[derive(Debug)]
pub enum Option<T> {
    None,
    Some(T),
}
