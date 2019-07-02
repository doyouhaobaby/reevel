use std::panic;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = panic::catch_unwind(|| {
        println!("hello world");
    });
    assert!(result.is_ok());

    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            println!("panic occurred `{}`, at `{}`", 
                location.file(), location.line());
        } else {
            println!("can not found location infor");
        }
    }));

    let result = panic::catch_unwind(|| {
        panic!("error");
    });
    assert!(result.is_err());
    println!("{}", sum(1, 2));
}
