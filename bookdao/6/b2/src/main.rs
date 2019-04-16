fn modify(mut v: Vec<i32>) -> Vec<i32> {
    v.push(8);
    v
}

fn main() {
    let v = vec![1, 2, 3];
    let v = modify(v);
    println!("{:?}", v);
}
