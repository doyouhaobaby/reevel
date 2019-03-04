fn main() {
    let arr: [i32; 5] = [1,2,3,4,5];
    assert_eq!(&arr, &[1,2,3,4,5]);
    assert_eq!(&arr[1..], &[2,3,4,5]);
    assert_eq!(&arr[1..], [2,3,4,5]);
    assert_eq!(&arr.len(), &5);
    assert_eq!(&arr.is_empty(), &false);

    let arr = &mut [1, 2, 3];
    arr[1] = 10;
    assert_eq!(arr, &[1, 10, 3]);

    let z = vec![1,2,3,4];
    assert_eq!(&z, &[1,2,3,4]);
}
