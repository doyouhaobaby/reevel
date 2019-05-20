fn main() {
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);

   // vec.insert(8, 8);thread 'main' panicked at 'assertion failed: index <= len',
}

