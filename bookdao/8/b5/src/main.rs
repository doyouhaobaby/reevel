fn main() {
    assert_eq!(true, 'f'.is_digit(16));
    assert_eq!(false, 'f'.is_digit(10));
    assert_eq!(Some(15), 'f'.to_digit(16));
    assert!('f'.is_lowercase());
    assert!(!'刘'.is_lowercase());
    assert!('F'.is_uppercase());
    assert!(!'f'.is_uppercase());
    println!("lowercase {:?}", 'I'.to_lowercase());

    //assert_eq!('i', 'I'.to_lowercase());// 报错
    //assert_eq!('F', 'f'.to_uppercase());// 报错

    for c in 'I'.to_lowercase() {
        println!("{}", c);
    }

    assert_eq!("i", 'I'.to_lowercase().to_string());
    assert_eq!("I", 'i'.to_uppercase().to_string());

    assert!(' '.is_whitespace());
    assert!(!'h'.is_whitespace());

    assert!('a'.is_alphabetic());
    assert!('刘'.is_alphabetic());
    assert!(!'1'.is_alphabetic());

    assert!('5'.is_alphanumeric());
    assert!('刘'.is_alphanumeric());
    assert!(!'!'.is_alphanumeric());
    
    assert!('5'.is_numeric());
    assert!('\n'.is_control());

    println!("hello {}", '\r'.escape_default());
}
