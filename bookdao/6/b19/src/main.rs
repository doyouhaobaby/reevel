fn bar(i: i32) -> fn(a: i32) -> i32 {
    fn inc(b: i32) -> i32 {
        b+i
        //can't capture dynamic environment in a fn item
    }
    inc
}

fn main() {
}
