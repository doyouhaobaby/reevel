fn main() {
    let out = 32;
    
    fn add (i: i32, j: i32) -> i32 {
        i + j
    }

    let closure_1 = |i: i32, j: i32| -> i32 {
        i + j + out
    };

    let closure_2 = |i, j| -> i32 {
        i + j + out
    };

    // can't capture dynamic environment in a fn item
    // fn error_fn (a: i32, b: i32) -> i32 {
    //     a + b + out
    // }

    assert_eq!(15, add(10, 5));
    assert_eq!(41, closure_1(4, 5));
    assert_eq!(41, closure_2(4, 5));
}
