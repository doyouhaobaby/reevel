fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        return x;
    }

    gcd(y, x%y)
}

fn main() {
    let t = gcd(60, 40);
    assert_eq!(20, t);
}
