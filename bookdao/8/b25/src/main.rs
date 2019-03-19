fn main() {
    let s = "hello\tworld\txxx";
    assert_eq!("hello world xxx", s.replace("\t", " "));
    let s = "hello\tworld\txxx\t";
    assert_eq!("hello world xxx", s.replace("\t", " ").trim());
    println!("hello world");

    let s = "hello world";
    assert_eq!("hello newld", s.replace("wor", "new"));
    let s = "hello world world world";
    assert_eq!("hello newld newld newld", s.replace("wor", "new"));
    assert_eq!("hello newld newld world", s.replacen("wor", "new", 2));
    let s = "hello 123456 world 456 xxx wrl";
    assert_eq!("hello newnewnew456 world 456 xxx wrl", s.replacen(char::is_numeric, "new", 3));
}
