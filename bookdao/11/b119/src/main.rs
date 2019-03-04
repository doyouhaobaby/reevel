use std::panic;
use std::thread::{Builder, current};
//use std::thread;

fn main() {
    let mut v  = vec![];

    for id in 0..10 {
        let thread_name = format!("child-{}", id);
        let s : usize = 1024 * 3;

        let builder = Builder::new()
            .name(thread_name)
            .stack_size(s);

        //println!("build {:?}", builder);

        let child = builder.spawn (move || {
           println!("child thread {}", id); 

           if 3 == id {
               panic::catch_unwind (|| {
                  panic!("thread error");
               });

               println!("{:?}", current());

               println!("in {} do sm", current().name().unwrap());
           }
        }).unwrap();

        // let child = thread::spawn(move || {
        //     println!("child thread {}", id);
        // });

        v.push(child);
    }

    for child in v {
        child.join();
    }

    println!("All done");
}


