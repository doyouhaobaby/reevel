extern crate b35;
use b35::*;
use std::env;
fn main() -> Result<(), String> {
    let filename = env::args().nth(1);
    match b35::run(filename) {
        Ok(n) => {
            println!("{:?}", n);
            return Ok(());
        },
        Err(e) => {
            return Err("1".to_string());
        }
    }
}