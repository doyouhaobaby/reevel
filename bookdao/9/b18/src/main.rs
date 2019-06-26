use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];
    let mut f = File::open(filename).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let mut sum = 0;
    for c in content.lines() {
        let n = c.parse::<i32>().unwrap();
        sum += n;
    }
    println!("{:?}", sum);
}
