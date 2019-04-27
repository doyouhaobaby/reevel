fn main() {
    let mut message = "hello".to_string();
    message.extend(&[' ', 'r', 'u', 's', 't']);
    assert_eq!(message, "hello rust".to_string())
}
