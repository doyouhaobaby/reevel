#[derive(Debug)]
struct S {
    i: i32,
}

fn f(ref s: S) {
   println!("{:p}", s);
   println!("{:?}", s);
}

fn main() {
    let s  = S {i: 32};
    f(s);
    //println!("{:?}", s); //^ value borrowed here after move
    //println!("{:p}", s);//the trait `std::fmt::Pointer` is not implemented for `S`
}
