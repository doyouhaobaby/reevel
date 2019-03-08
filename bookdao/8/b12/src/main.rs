fn main() {
    let mut message = String::from("hello");
    message.extend(&[' ', 'w']);
    assert_eq!(message, "hello w");
    message.extend(['o', 'r'].iter());
    assert_eq!(message, "hello wor");
    message.extend("xiaoniuge".chars());
    assert_eq!("hello worxiaoniuge", message);
    message.extend("x y z".split_whitespace());
    assert_eq!(message, "hello worxiaoniugexyz");
}
