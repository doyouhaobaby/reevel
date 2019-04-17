use std::ops::Mul;

fn square<T>(x: T, y: T) -> T 
where T : Mul<T, Output=T>{
    x * y
}

fn main() {
    let a: i32 = square(37, 41);
    let b: f64 = square(37.2, 55.8); 
    println!("{}", a);
    println!("{}", b); 
}
