fn main() {
   assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
   assert_eq!("hello world".trim_matches('d'), "hello worl");
   assert_eq!("1111helloworld55".trim_matches(char::is_numeric), "helloworld");
   let x: &[char] = &['1', '5', '6'];
   assert_eq!("1116helloworld55".trim_matches(x), "helloworld");
   assert_eq!("111helloworld55".trim_start_matches(char::is_numeric), "helloworld55");
   assert_eq!("111helloworld55".trim_end_matches(char::is_numeric), "111helloworld");
   
   let x: &[char] = &['1', '5', '6'];
   assert_eq!("111helloworld55".trim_end_matches(x), "111helloworld");
   assert_eq!("111helloworld55".trim_start_matches(x), "helloworld55");
   assert_eq!("111helloworldxxx55555".trim_end_matches(|c| 
       c == '5' || c == '1'
   ), "111helloworldxxx");
}
