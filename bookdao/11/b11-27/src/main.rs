use std::sync::RwLock;

fn main() {
    //println!("Hello, world!");

    let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        println!("r1 {}", r1);
        assert_eq!(*r1, 5);

        let r2 = lock.read().unwrap();
        println!("r2 {}", r2);
        assert_eq!(*r1, 5);
    }

    {
        let mut w1 = lock.write().unwrap();
        *w1 += 1;
        assert_eq!(*w1, 6);
    }

    println!("Done");
}
