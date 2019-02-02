fn main() {
    let a: &'static str = "Rust 是一门优雅的语言";// PHP 静态属性
    println!("{}", a);

    let ptr = a.as_ptr();
    println!("{:p}", ptr);

    let l = a.len();
    assert_eq!(29, l);

    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, l);
        std::str::from_utf8(slice)
    };

    println!("{:?}", s);

    assert_eq!(s, Ok(a));
}
