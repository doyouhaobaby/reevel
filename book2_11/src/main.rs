fn is_true () -> bool {
    true
}

fn true_make () -> fn() -> bool {
    is_true
}

fn main() {
    assert_eq!(true, is_true());
    assert_eq!(true, true_make()());
}
