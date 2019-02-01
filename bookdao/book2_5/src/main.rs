fn main() {
    let place1 = "hello 1";// 引用，借用
    let place2 = "hello 2".to_string();// 拥有所有权
    let other = place1;
    println!("{:?}", other);
    println!("{}", place1);
    println!("{}", place2);

    let other = place2;
    println!("{:?}", place2);
}
