fn foo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() % 2 == 0 {
        return a
    }

    return b
} 

fn main() {
    let a = "hello world";
    let c;
    let b = "foo bar";
    c = foo(&a, &b);
    println!("{}", c);

    let a = "hello world1";
    let c;
    let b = "foo bar";
    c = foo(&a, &b);
    println!("{}", c);

    let a = "hello world1".to_string();
    let c;
    let b = "foo bar".to_string();
    c = foo(&a, &b);
    println!("{}", c);
}
