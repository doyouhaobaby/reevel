fn main() {
    let x = true;
    println!("{}", x);

    let y: bool = false;
    println!("{}", y);

    assert_eq!(x as i32, 1);
    assert_eq!(y as i32, 0);

    let z = 55;

    if z > 1 {
        println!("z 大于 1");
    }
}
