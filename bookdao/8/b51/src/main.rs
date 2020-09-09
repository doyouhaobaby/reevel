fn main() {
    pick([1, 2, 3]);
    pick([3,2,9]);
    pick([6,6,6]);
}

fn pick(arr: [i32; 3]) {
    match arr {
        [_, _, 3] => println!("ends with 3"),
        [a, 2, c] => println!("{:?}, 2, {:?}", a, c),
        [_,_,_] => println!("pass"),
    }
}
