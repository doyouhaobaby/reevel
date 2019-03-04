fn main() {
    let mut a = vec![1,2,3,4,5,6,7];

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
