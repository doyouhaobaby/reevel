trait Add<RHS, Output> {
    fn my_add(self, rhs: RHS) -> Output;
}

impl Add<i32, i32> for i32 {
    fn my_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl Add<u32, i32> for u32 {
    fn my_add(self, rhs: u32) -> i32 {
        (self + rhs) as i32
    }
}

fn main() {
    let a: i32 = 5;
    let b: i32 = 6;
    assert_eq!(11, a.my_add(b));

    let c: u32 = 55;
    let d: u32 = 5;
    assert_eq!(60, c.my_add(d));
}
