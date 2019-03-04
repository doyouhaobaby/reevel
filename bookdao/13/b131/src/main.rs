fn main() {
    unsafe {
        let mut a = "hello world";
        let b = &a;
        let c = &mut a;

        println!("xx{}", b);
    }
}
