fn while_true (x: i32) -> i32 {
    while true {
        return x + 5;
    }
    x // 没有这行要报错,rust while 条件可真可假的无法判断返回值为 i32
}

fn main() {
    let y = while_true (5);
    assert_eq!(y, 10);
}
