use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
struct Node {
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>,
}
impl Drop for Node {
    fn drop(&mut self) {
        println!("dropping");
    }
}
fn main() {
   let first = Rc::new(RefCell::new(Node {next: None, head: None}));
   let second = Rc::new(RefCell::new(Node {next: None, head: None}));
   let third = Rc::new(RefCell::new(Node {next: None, head: None}));

    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(third.clone());
    third.borrow_mut().head = Some(Rc::downgrade(&first));
}

/*
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/b52`
dropping
dropping
dropping
*/