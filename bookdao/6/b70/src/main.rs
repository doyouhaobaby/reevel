fn main() {
    let mut arr = [1, 2, 3];
    for i in arr.iter_mut() {
        *i *= 2;
    }

    assert_eq!([2, 4, 6], arr);
}