use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let a = Rc::new("hello".to_string());
    let b = Arc::new(vec![1, 2, 3]);
    println!("{:?}", *a);
    println!("{:?}", *b);
    // error[E0507]: cannot move out of an `Rc`
    // let d = *a;
    // error[E0507]: cannot move out of an `Arc`
    // let r = *b;

    let c = Rc::new("hello world");
    let e = *c;
    println!("{:?}", e);
}
