fn main() {
    let s = String::from("foobar");
    let s: String = s.chars().enumerate().map(|(i, c)| {
        //assert_eq!((), c);

        if i % 2 == 0 {
            c.to_lowercase().to_string()
        } else {
            c.to_uppercase().to_string()
        }
    }).collect();

    assert_eq!("fOoBaR", s);
}
