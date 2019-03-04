use std::mem::transmute;

fn main() {
    //println!("Hello, world!");
    let x: &i32;
    {
        let a = 12;
        let ptr = &a as *const i32;
        x = unsafe { transmute::<*const i32, &i32>(ptr)};
    }
    println!("hello {}", x);
}
