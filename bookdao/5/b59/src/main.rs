use std::thread;

fn main() {
    let mut a = vec![1, 2, 3];

    for _i in 0..3 {
        thread::spawn(move || {
            a.push(5)
        });
    }

    println!("done");
}
