use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut handles = Vec::with_capacity(5); // 创建存放 5 条数据的 vec
    let barrier = Arc::new(Barrier::new(3));

    for _ in 0..5 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before await");

            c.wait();

            println!("after await");
        }));
    }

    for v in handles {
        v.join().unwrap();
    }

    println!("Done");
}
