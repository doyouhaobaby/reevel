fn main() {
    let mut a  = String::with_capacity(3);
    a.insert(0, 'a');
    a.insert(1, 'b');
    a.insert(2, 'c');
    assert_eq!("abc", a);
    a.insert_str(2, "efg");
    assert_eq!("abefgc", a);
}
