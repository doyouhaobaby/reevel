fn main() {
    let arr: [i32; 3] = [1,2,3];
    assert_eq!(1, arr[0]);
    println!("{:?}", arr);

    let mut b = [4,5,6];

    println!("{:?}", b);

    b[2] = 555;
    assert_eq!(555, b[2]);
    println!("{:?}", b);

    let zz = [0; 10];
    assert_eq!(zz[3], 0);
    assert_eq!(zz[5], 0);
    assert_eq!(10, zz.len());

   // println!("{}", zz[333]);
   // index out of bounds: the len is 10 but the index is 333
}
