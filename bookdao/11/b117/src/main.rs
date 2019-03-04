use std::thread;

fn main() {
    //println!("Hello, world!");

    let mut v = vec![];

    for id in 0..10 {
        let child = thread::spawn(move || {
            println!("in child {}", id);
        });
        v.push(child);
    }

    println!("in main befero");

    for child in v {
        child.join();
    }

    println!("in mian after");
}
