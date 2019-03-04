//#![feature(never_type)]
// fn foo() -> i32 {
//   let x: != {
//       return 32
//   };
// }

fn main() {
    let a: Option<u32> = Some(44);

    println!("{:?}", a);
    match a {
        Some(n) => println!("{}", n),
        None => println!("none"),
    };

    let z = match a {
        Some(n) => n,
        None => panic!("xxxx"),
    };

    println!("{:?}", z);
}
