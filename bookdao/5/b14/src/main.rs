fn main() {
    let a = Some("hello".to_string());
    match a {
        Some(s) => println!("{}", s),
        _ => println!("nothing"),
    }

    println!("{:?}", a);
/*
    src/main.rs:8:22
  |
4 |         Some(s) => println!("{}", s),
  |              - value moved here
...
8 |     println!("{:?}", a);
  |                      ^ value borrowed here after partial move
  |
  */
}
