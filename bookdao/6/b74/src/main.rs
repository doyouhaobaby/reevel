fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let c1 = arr1.iter().map(|x| 2*x).collect::<Vec<i32>>();
    println!("{:?}", c1);
    assert_eq!(&c1[..], [2, 4, 6, 8, 10]);

    let arr2 = ["1", "2", "3", "h"];
    println!("{:?}", arr2);
    let c2 = arr2.iter().filter_map(|x| x.parse().ok()).
        collect::<Vec<i32>>();
    println!("{:?}", c2);
    assert_eq!(&c2[..], [1, 2, 3]);
    assert_eq!(c2, [1, 2, 3]);

    let arr3 = ['a', 'b', 'c'];
    for (idx, val) in arr3.iter().enumerate() {
        println!("idx: {:?}, val: {}", idx, val.to_uppercase());
    }
}
