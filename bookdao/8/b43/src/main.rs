fn main() {
    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&1110));
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(!v.starts_with(&[40, 40]));
    assert!(v.ends_with(&[30]));
    assert!(!v.ends_with(&[440]));
    assert!(v.ends_with(&[]));
    assert!(v.starts_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));
    assert!(v.ends_with(&[]));
}
