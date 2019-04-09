use std::rc::Rc;

fn main() { 
    let x = Rc::new(45);
    let x1 = x.clone();
    let x2 = x.clone();

    println!("{:?}", Rc::strong_count(&x));

    println!("{:?}", Rc::weak_count(&x));

    let w = Rc::downgrade(&x);
    println!("{:?}", Rc::weak_count(&x));

    let y3 = &*x;
    println!("{:?}", Rc::strong_count(&x));
    println!("{:?}", 100 - *x);
}
