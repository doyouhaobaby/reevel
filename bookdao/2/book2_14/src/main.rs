fn closure_as_args<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn main() {
    let i = 3;
    let m = 5;

    assert_eq!(closure_as_args(|| i + m), 8);
    assert_eq!(closure_as_args(|| i*m), 15);
}
