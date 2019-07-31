#![allow(unused)]
fn main() {
    let mut v = [-51, 4, 1, -3, 2];
    v.sort();
    assert_eq!(v, [-51,-3, 1,2,4]);
    assert!(v == [-51,-3,1,2,4]);
    v.sort_by(|a, b| a.cmp(b));
    assert_eq!(v, [-51,-3, 1,2,4]); 
    assert!(v == [-51,-3,1,2,4]);
    v.sort_by(|a,b| b.cmp(a));
    assert!(v == [4,2,1,-3,-51]);
    //v.sort_by_key(|k| k.abs());
    //assert!(v == [1,2,-3,4,-15]);
    //v.sort_by_key(|k| k.abs());

    let mut v = [-5i32, 4, 1, -3, 2];
    v.sort_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);

    //assert_eq!(-5, "string");
    //assert_eq!(-3, "string");

    // error[E0599]: no method named `abs` found for type `&{integer}` in the current scope
    // let mut v = [-5, 4, 1, -3, 2];
    // v.sort_by_key(|k| k.abs());
    // assert!(v == [1, 2, -3, 4, -5]);
}
