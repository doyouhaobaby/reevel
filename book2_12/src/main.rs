//#![feature(const_fn)]
//const fn is unstable
const fn init_len() -> usize {
    return 5;
}

fn main() {
    let arr = [0; init_len()];
    println!("{:?}", arr);
}