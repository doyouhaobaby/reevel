fn main() {
    let a = "hello 小牛哥 world";
    let s = a.split(|c| {
        (c as u32) >= (0x4E00 as u32) && (c as u32) <= (0x9FA5 as u32) // 中文范围
    }).collect::<Vec<&str>>();

    assert_eq!(s, &["hello ", "", "", " world"]);
    assert_eq!(s, ["hello ", "", "", " world"]);

    let a = "hello world";
    let s = a.split(|c|
        c == 'e' || c == 'r'
    ).collect::<Vec<&str>>();
    assert_eq!(s, ["h", "llo wo", "ld"]);

    let a = "hello world for rust";
    let s = a.split(|c| 
        c == ' '
    ).collect::<Vec<&str>>();
    assert_eq!(s, ["hello", "world", "for", "rust"]);

    let a = "a.b.c.";
    let s = a.split('.').collect::<Vec<&str>>();
    assert_eq!(s, ["a", "b",   "c", ""]);

    let a = "a.b.c.";
    let s = a.split_terminator('.').collect::<Vec<&str>>();
    assert_eq!(s, ["a", "b", "c"]);

    let a = "a.b.c.d.e...";
    let s = a.splitn(2, ".").collect::<Vec<&str>>();
    assert_eq!(s, ["a", "b.c.d.e..."]);
}