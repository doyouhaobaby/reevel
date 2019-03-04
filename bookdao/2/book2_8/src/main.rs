pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if num % 5 == 0 {
        return "fizz".to_string();
    } else if num % 3 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}

fn main() {
    assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
    assert_eq!(fizz_buzz(5), "fizz".to_string());
    assert_eq!(fizz_buzz(3), "buzz".to_string());
    assert_eq!(fizz_buzz(11), "11".to_string());
}
