fn main() {
    let a = Box::new("hello");
    let b = Box::new("Rust".to_string()); // 转移了所有权
    let c = *a;
    let d = *b;
    println!("{:?}", a);
    // error[E0382]: borrow of moved value: `b`
    // println!("{:?}", b);
}
