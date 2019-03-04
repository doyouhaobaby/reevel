fn main() {
    let x = 'x';
    let x = 'y';
    println!("{}", x);

    println!("{}", '\'');
    println!("{}", '\t');
    println!("{}", '\n');

    assert_eq!('\x2A', '*');
    assert_eq!('\x25', '%');
    assert_eq!('%' as i32, 37);
}
