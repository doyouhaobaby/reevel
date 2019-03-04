#![feature(allocator_api, alloc_layout_extra)]
use std::alloc::{GlobalAlloc,System, Layout};
use std::ptr;
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

struct MyBox<T> {
    v: *const T,
}

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        unsafe {
            let p = System.alloc(Layout::array::<T>(1).unwrap());
            let p = p as *mut T;
            ptr::write(p, t);

            MyBox {
                v: p
            }
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop (&mut self) {
        unsafe {
            // let p = self.v as *mut _;
            // let b = Layout::array::<T>(mem::align_of::<T>()).unwrap();

            // System::dealloc(
            //     p,
            //     b
            // );
        }
    }
}

fn f1() {
    let (x, y);
    x = Hello::new("hello world", 13);
    y = WrapBox::new(Hello::new("hahaha", &x));
}

fn f2() {
    {
        let (x1, y1);
        x1 = Hello::new("hello", 13);
        y1 = MyBox::new(Hello::new("world", &x1));
    }

    // {

    // }
}

fn main() {
   //f1();
   f2();
}
