fn hello() {
    println!("hello function pointer");
}

fn main() {
    let fn_p: fn() = hello;
    println!("{:?}", fn_p);
    let o = hello;
    //println!("{:p}", o);//^ the trait `std::fmt::Pointer` is not implemented for `fn() {hello}`
    hello();
    o();
    fn_p();
    (fn_p)();
}
