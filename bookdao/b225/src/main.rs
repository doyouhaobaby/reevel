fn main() {
    let num = 4.2222;
    println!("{}", num);

    let _a = 42222u32;
    let _b: u32 = 444;
    let c = 0x2A;
    println!("{}", c);

    let num = 0o106;
    println!("{}", num);

    let num = 0b1001_1101;
    println!("{}", num);

    assert_eq!(b'*', 42u8);
    assert_eq!(b'\'', 39u8);
    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2., 2.0f64);

    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);
}
