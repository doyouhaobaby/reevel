fn main() {
    let test = (0, 2);
    dbg! (&test);

    let ref_value = & 4;
    let value = 5;
    let mut value2 = 6;

    match test {
        (0, y) => println!("y is {}", y),
        _ => println!("x"),
    }

    match ref_value {
        &val => println!("解构 {}", val),
    }

    match &ref_value {
        val => println!("test {}", val),
    }

    match value {
        ref r => println!("xxxx {:?}", r),
    }

    match value2 {
         ref mut r => {
            *r + 10;
            println!("xxx {:?}", r);
        },
    }
}
