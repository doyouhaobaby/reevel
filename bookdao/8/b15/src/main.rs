//use std::ascii::{AsciiExt};

fn main() {
    let foo = "hello world".to_string();
    let mut foo = foo.into_bytes();

    (0..foo.len()).for_each(|i| {
        if i % 2 == 0 {
            foo[i] = foo[i].to_ascii_lowercase();
        } else {
            foo[i] = foo[i].to_ascii_uppercase();
        }
    });

    assert_eq!("hElLo wOrLd", String::from_utf8(foo).unwrap());
}
