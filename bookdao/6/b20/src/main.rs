fn counter(i: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |n: i32| n+i)
}

fn main() {
    let f = counter(4);
    assert_eq!(13, f(9));
}
