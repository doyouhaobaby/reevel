fn main() {
    for n in 0..100 {
        if (n % 15 == 0) {
            println!("foo bar");
        } else if (n % 3 == 0) {
            println!("foo");
        } else if (n % 5 == 0) {
            println!("bar");
        } else {
            println!("{}", n);
        }
    }
}
