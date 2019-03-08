fn main() {
    let a = "hello world";
    let (first, seccode) = a.split_at(6);
    assert_eq!(first, "hello ");
    assert_eq!(seccode, "world");
    //let (first, seccode) = a.split_at(555);// 位置不存在，线程崩溃
}
