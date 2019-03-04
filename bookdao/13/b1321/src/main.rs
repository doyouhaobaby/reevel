fn foo <'a> (input: *const u32) -> &'a u32 {
    unsafe {
        return &*input
    }
}

fn main() {
    //println!("Hello, world!");
    let x;
    {
        let y = 42;
        x = foo(&y);
    }

    println!("xxx {}", x); // xxx 42 :cargo run
    // xxx 182989416: cargo run --release
}

