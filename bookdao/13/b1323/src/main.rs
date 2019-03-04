use std::fmt;

#[derive(Copy, Clone, Debug)]
enum State {
    InValid,
    Valid,
}

#[derive(Debug)]
struct Hello<T: fmt::Debug>(&'static str, T, State);

impl<T: fmt::Debug> Hello<T> {
    fn new(name: &'static str, t2: T) -> Self {
        Hello(name, t2, State::Valid)
    }
}

impl<T: fmt::Debug> Drop for Hello<T> {
    fn drop(&mut self) {
        println!("drop hello({}, {:?}, {:?})", self.0, self.1, self.2);
        self.2 = State::InValid
    }
}

struct WrapBox<T> {
    v: Box<T>,
}

impl <T> WrapBox<T> {
    fn new(t: T) -> Self {
        WrapBox {v: Box::new(t)}
    }
}

fn main() {
    let (x, y);
    x = Hello::new("hello world", 13);
    y = WrapBox::new(Hello::new("hahaha", &x));
}
