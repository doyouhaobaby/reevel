fn main() {
    let vec: Vec<i32> = vec![1, 2, 3];
    assert_eq!(vec[..], [1, 2, 3]); // 数组和数组比较
                                    //assert_eq!(vec[..], &[1,2,3]);// 数组和数组引用.报错
                                    //    assert_eq!(&vec[..], &[1,2,3]); // 切片和切片
                                    //    assert_eq!(&vec[..], [1,2,3]);
                                    //    assert_eq!(vec[1..], [1,2,3]);
    let _b = "xxx";
    //println!("{:?}", &[1,2,3]);
    //[i32] == &[{integer}; 3]

    // match ( & ( vec[..] ) , & ( &[1,2,3] ) ) {
    //     ( left_val , right_val ) => {
    //         println!("xx{:?}x", left_val);
    //         println!("xx{:?}x", right_val);
    //         if ! ( * left_val == * right_val ) {
    //         panic ! ("assertion failed: `(left == right)`");
    //     }
    // }
    // }

    // match ( & ( &vec[..] ) , & ( &[1,2,3] ) ) {
    //     ( left_val , right_val ) => {
    //         println!("xx{:?}x", left_val);
    //         println!("xx{:?}x", right_val);
    //         if ! ( * left_val == * right_val ) {
    //         panic ! ("assertion failed: `(left == right)`");
    //     }
    // }
    // }

    match (&(&&vec[..]), &(&[1, 2, 3])) {
        (left_val, right_val) => {
            println!("xx{:?}x", left_val);
            println!("xx{:?}x", right_val);
            if !(*left_val == *right_val) {
                panic!("assertion failed: `(left == right)`");
            }
        }
    }

    //hello(&(&vec[..]));

    //    fn hello (a : i32) {
    //        a
    //    }
}

// fn hello <T, N>(a: T, b: N) {
//     match ( & ( a ) , & ( b ) ) {
//         ( left_val , right_val ) => {
//             if ! ( * left_val == * right_val ) {
//             panic ! (
//                 "assertion failed: `(left == right)` \
//                             (left: ``, right: ``)"
//             /*, * left_val , * right_val */)
//         }
//     }
//     }
// }
