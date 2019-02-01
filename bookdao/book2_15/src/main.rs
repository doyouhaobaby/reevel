fn two_times () -> impl Fn(i32) -> i32 {
    let i  = 32;
    move |j| j * i
}

fn main() {
    let result = two_times();
    assert_eq!(64, result(2));
}
