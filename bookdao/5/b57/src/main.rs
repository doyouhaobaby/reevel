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
    let token  = Token::new("abc123");
    let token =  Token::new("hello".to_string());
    thread::spawn(move || {
        println!("token: {:?}", token);
    }).join().unwrap();

    println!("done");
}
