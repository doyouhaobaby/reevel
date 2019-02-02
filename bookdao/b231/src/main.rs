fn main() {
    let mut x = 2;
    let p_x = &mut x as *mut i32;
    println!("{:p}", p_x);
    let y = Box::new(4);
    let p_y = &*y as *const i32;
    println!("{:p}", p_y);
    unsafe {
        *p_x += *p_y;
    }
    assert_eq!(6, x);
}
