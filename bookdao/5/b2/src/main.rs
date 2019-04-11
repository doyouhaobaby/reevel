fn main() {
    let a = Box::new("xx".to_string());
    println!("{}", *a);
    let c = *a;
    let b = a;
    println!("{}", *b);
}
