fn main() {
    let four: i32 = "4".parse().unwrap();
    assert_eq!(4, four);
    let four = "4".parse::<i32>();
    assert_eq!(Ok(4), four);
}
