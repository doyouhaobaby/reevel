use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let p_c = pair.clone();

    thread::spawn(move || {
        println!("{:?}", p_c);
        println!("{:?}", *p_c);
        let &(ref lock, ref cvar) = &*p_c;
        println!("{:?}", lock);
        println!("{:?}", cvar);
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("here");
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("{}", started);
        started = cvar.wait(started).unwrap();
        println!("{}", started);
    }
    
    println!("Done");
}
