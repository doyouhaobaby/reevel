fn main() {
    let mut a = vec![1,2,3,4,5,6];

    loop {
        match a.pop() {
            Some(i) => println!("{}", i),
            None => break,
        }
    }
}
