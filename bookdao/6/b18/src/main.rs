fn bar() -> fn(a: i32) -> i32 {
    fn inc(b: i32) -> i32 {
        b+1
    }
    inc
}

fn main() {
    let a = bar();
    assert_eq!(2, a(1));
    assert_eq!(4, a(3));
}
