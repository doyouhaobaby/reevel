fn swap((x, y): (&str, i32)) -> (i32, &str) {
   (y, x) 
}

fn main() {
    let t = ("hello", 99);
    let t2 = swap(t);
    assert_eq!(t2, (99, "hello"));
}
