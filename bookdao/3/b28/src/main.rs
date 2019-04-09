use std::ops::Add;

// impl doesn't use types inside crate
// 触发孤儿规则 Orphan rule
// 
// 
impl Add<u64> for u32 {
    type Output = u64;

    fn add(self, other: u64) -> Self::Output {
        (self as u64) + other
    }
}

fn main() {
    let a = 1u32;
    let b = 5u64;

    assert_eq!(6, a.add(b));
}
