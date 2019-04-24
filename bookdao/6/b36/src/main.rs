fn main() {
    let s = "hello".to_string();
    let c = move || { println!("{:?}", s); };
    c();
    c();
    //println!("{:?}", s);// value borrowed here after move
}
