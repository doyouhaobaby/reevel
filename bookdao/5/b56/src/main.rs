use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            println!("{}", v);
            input.to_mut()[i] = -v;
        }
    }
}

fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    lst.iter().fold(0, |acc, &n| acc + n)
}

fn main() {
    let s1 = [1,2,3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("{:?}", i1);
    let s1 = [-1,2,3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("{:?}", i1);

    let mut v1 = Cow::from(vec![1, 2, 3, -4]);
    abs_all(&mut v1);

    let a = [1,2,3];
    let r = abs_sum(&a);
    println!("{}", r);
    let a = [1, -4];
    let r = abs_sum(&a);
    println!("{}", r);
}
