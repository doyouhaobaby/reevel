fn main() {
    let a = ("hello".to_string(), "world".to_string());
    let b = a;
    //print!("{:?}", a);//value borrowed here after move

    let a = (1, 2, 3);
    let b = a;
    print!("{:?}", a);
}
