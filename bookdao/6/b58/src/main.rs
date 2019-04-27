fn main() {
    let v = vec![1, 2, 3, 4];
    {
        let mut x = v.into_iter();
        loop {
            match x.next() {
                Some(i) => {
                    println!("{}", i);
                },
                None => break,
            }
        }
    }
}
