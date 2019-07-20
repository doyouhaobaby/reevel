fn main() {
    let mut vec = Vec::with_capacity(3);
    assert_eq!(vec.capacity(), 3);
    assert_eq!(vec, []);
    for i in 0..3 {
        vec.push(i);
    }
    assert_eq!(vec, [0,1,2]);
    vec.truncate(0);
    assert_eq!(vec, []);
    assert_eq!(vec.capacity(), 3);

    for i in 0..3 {
        vec.push(i);
    }
    assert_eq!(vec, [0,1,2]);
    vec.clear();
    assert_eq!(vec, []);

    vec.shrink_to_fit();
    assert_eq!(0, vec.capacity());
    for i in 0..3 {
        vec.push(i);
        println!("{:?}", vec.capacity());
    }
}
