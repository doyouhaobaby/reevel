fn main() {
    let mut a = String::from("hello world");
    println!("{:p}", a.as_ptr());// 堆中字节序列的指针地址
    println!("{:p}", &a);// 字符串变量在栈中指针地址
    assert_eq!(11, a.len());
    assert_eq!(11, a.capacity());// 获取空间
    a.reserve(11);// 增加控件
    assert_eq!(22, a.capacity());
}
