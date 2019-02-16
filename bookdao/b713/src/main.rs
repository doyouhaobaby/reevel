fn main() {
    println!("Hello, world!");

    
    println!("{:?}", Color::Black);
}

#[derive(Debug)]
enum Color {
    Red,
    Yellow,
    Black,
}
