use std::borrow::Cow;
use std::thread;

#[derive(Debug)]
struct Token <'a> {
    raw: Cow<'a, str>,
}

impl<'a> Token<'a> {
    fn new<S>(raw: S) -> Token<'a> 
    where 
        S: Into<Cow<'a, str>>,
    {
        Token {raw: raw.into()}
    }
}

fn main() {
    let raw = String::from("hello world");
    let s = &raw[..];
    let token = Token::new(s);
    //`raw` does not live long enough
    thread::spawn(move || {
        println!("token: {:?}", token);
    }).join().unwrap();
}
