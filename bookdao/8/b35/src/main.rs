fn main() {
    let s: String = format!("{}world", "hello");
    assert_eq!(s, "helloworld");
    assert_eq!(format!("{:5}", "helloworld"), "helloworld");
    assert_eq!(format!("{:12}", "helloworld"), "helloworld  ");
    assert_eq!(format!("{:5.3}", "helloworld"), "hel  ");
    assert_eq!(format!("{:<12}", "helloworld"), "helloworld  ");
    assert_eq!(format!("{:^12}", "helloworld"), " helloworld ");
    assert_eq!(format!("{:>12}", "helloworld"), "  helloworld");
    assert_eq!(format!("{:*>12}", "helloworld"), "**helloworld");
}
