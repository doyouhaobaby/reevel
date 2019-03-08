fn main() {
    let string: String = String::new();
    assert_eq!("", string);

    let string = String::from("hello world");
    assert_eq!("hello world", string);

    let string = String::with_capacity(10);
    assert_eq!("", string);

    let str : &'static str = "hello xiaoniuge";
    println!("chars - {:?}", str.chars());
    println!("chars - {:?}", str.chars().filter(|c| !c.is_whitespace()));
    let string: String = str.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!("helloxiaoniuge", string);

    let string: String = str.to_owned();
    assert_eq!("hello xiaoniuge", string); 

    let string: String = str.to_string();
    assert_eq!("hello xiaoniuge", string);

    let str = &string[11..15];// 字符串切片
    assert_eq!("iuge", str);
}
