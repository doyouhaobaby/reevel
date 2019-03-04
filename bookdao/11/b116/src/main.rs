use std::thread;

fn main() {
    //println!("Hello, world!");

    let child = thread::spawn(move || {
       for _ in 0..20 {
           unsafe_seq();
           unsafe {
              println!("child: {}", v);
           }
       } 
    });

    for _ in 0..20 {
        unsafe_seq();
        unsafe {
            println!("main: {}", v);
        }
    }  
}

static mut v: i32 = 0;

fn unsafe_seq() -> i32 {
    unsafe {
        v += 1;
        v
    }
}

