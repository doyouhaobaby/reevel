fn main() {
    let (a, b) = (1, 2);

    cmp(a, b);
}

macro_rules! unless {
    ($arg:expr) => (
        if $arg {
            println!("a > b");
        } else {
            println!("a < b");
        }
    )
}

fn cmp (a : i32, b: i32) {
    unless! (a > b);
    // if a > b {
    //     println!("a > b");
    // } else {
    //     println!("a < b");
    // }
}
