use std::ops::Mul;

fn square<T>(x: T, y: T) -> T 
where T : Mul<T, Output=T>{
    x * y
}

fn main() {
    let a = square::<u32>(37, 41);
    let b = square::<f32>(37.2, 55.8); 
    println!("{}", a);
    println!("{}", b); 
}
