fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{:p}", &a);

    let b = &a;
    println!("{:p}", b);
    println!("{:?}", *b);

    let mut c = vec![1, 2, 3, 4];
    println!("{:?}", c);

    let d = &mut c;
    println!("{:p}", &d);
    println!("{:?}", *d);

    d.push(5);

    println!("{:?}", d);
    // println!("{:?}", c);// cannot borrow `c` as immutable because it is also borrowed as mutable

    let e = &41;
    assert_eq!(41, *e);
}
