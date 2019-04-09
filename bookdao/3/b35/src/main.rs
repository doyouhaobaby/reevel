use std::ops::Add;

fn sum<T: Add<T, Output=T>> (a: T, b: T) -> T {
    a + b
}

fn main() {
    assert_eq!(sum(5, 3), 8);
    //help: the trait `std::ops::Add` is not implemented for `&str`
    //sum("xxx", "xxxx");
}
